// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! This module implements lambda lifting, rewriting function definitions
//! in the global environment.
//!
//! Currently lambda lifting is performed only in selected cases:
//!
//! - Inside of specification expressions;
//! - For a lambda argument of a regular (not inline) function
//!
//! Lambda lifting rewrites lambda expressions into construction
//! of *closures*. A closure refers to a function and contains a partial list
//! of arguments for that function, essentially currying it. Example:
//!
//! ```
//! let c = 1;
//! vec.any(|x| x > c)
//! ~~~>
//! let c = 1;
//! vec.any(Closure(lifted, c))
//! where
//!   fun lifted(c: u64, x: u64): bool { x > c }
//! ```
//!
//! The lambda lifting in this module also takes care of patterns as lambda arguments.
//! Example:
//!
//! ```
//! let c = 1;
//! vec.any(|S{x}| x > c)
//! ~~~>
//! let c = 1;
//! vec.any(Closure(lifted, c))
//! where
//!   fun lifted(c: u64, arg$2: S): bool { let S{x} = arg$2; x > y }
//! ```

use itertools::Itertools;
use move_binary_format::file_format::Visibility;
use move_model::{
    ast::{Exp, ExpData, Operation, Pattern, TempIndex},
    exp_rewriter::ExpRewriterFunctions,
    model::{FunId, FunctionEnv, GlobalEnv, Loc, NodeId, Parameter, TypeParameter},
    symbol::Symbol,
    ty::Type,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
};

#[derive(Debug, Clone, Default)]
pub struct LambdaLiftingOptions {
    /// Whether to include inline functions, both definitions and arguments of calls.
    include_inline_functions: bool,
}

/// Performs lambda lifting for all target modules in the environment.
pub fn lift_lambdas(options: LambdaLiftingOptions, env: &mut GlobalEnv) {
    // Go over target modules one by one. Since in each iteration
    // we need to mutate the module, iterate over a vector of plain ids.
    for module_id in env
        .get_target_modules()
        .into_iter()
        .map(|me| me.get_id())
        .collect_vec()
    {
        let module = env.get_module(module_id);
        let mut updated_funs = BTreeMap::new();
        let mut new_funs = vec![];
        for fun in module.get_functions() {
            if fun.is_inline() && !options.include_inline_functions || fun.is_native_or_intrinsic()
            {
                continue;
            }
            let def = fun.get_def().expect("has definition");
            let mut lifter = LambdaLifter {
                options: &options,
                fun_env: &fun,
                lifted: vec![],
                scopes: vec![],
                free_params: BTreeMap::default(),
                free_locals: BTreeMap::default(),
                exempted_lambdas: BTreeSet::default(),
            };
            let new_def = lifter.rewrite_exp(def.clone());
            if def != &new_def {
                new_funs.append(&mut lifter.lifted);
                updated_funs.insert(fun.get_id(), new_def);
            }
        }
        // Now that we have processed all functions and released
        // al references to the env, mutate it.
        for (fun_id, new_def) in updated_funs {
            env.set_function_def(module_id.qualified(fun_id), new_def)
        }
        for ClosureFunction {
            loc,
            fun_id,
            type_params,
            params,
            result_type,
            def,
        } in new_funs
        {
            env.add_function_def(
                module_id,
                fun_id.symbol(),
                loc,
                Visibility::Private,
                type_params,
                params,
                result_type,
                def,
            )
        }
    }
}

/// Structure which is used to rewrite one function definition,
/// using the `ExpRewriterFunctions` trait. During
/// traversal of an expression, on ascent all the used but
/// so far unbound parameters and locals are stored here.
/// These are the ones which need to be captured in a closure.
struct LambdaLifter<'a> {
    /// The options for lambda lifting.
    options: &'a LambdaLiftingOptions,
    /// Function being processed.
    fun_env: &'a FunctionEnv<'a>,
    /// The generated closure functions.
    lifted: Vec<ClosureFunction>,
    /// Local name scopes.
    scopes: Vec<BTreeSet<Symbol>>,
    /// Any unbound parameters names used so far.
    free_params: BTreeMap<TempIndex, NodeId>,
    /// Any unbound locals used so far.
    free_locals: BTreeMap<Symbol, NodeId>,
    /// NodeId's of lambdas which are parameters to inline functions
    /// if those should be exempted. Pushed down during descend.
    exempted_lambdas: BTreeSet<NodeId>,
}

/// A new function to be created in the global env.
struct ClosureFunction {
    loc: Loc,
    fun_id: FunId,
    type_params: Vec<TypeParameter>,
    params: Vec<Parameter>,
    result_type: Type,
    def: Exp,
}

impl<'a> LambdaLifter<'a> {
    fn gen_local_name(&mut self) -> Symbol {
        unimplemented!()
    }

    fn gen_closure_function_name(&mut self) -> Symbol {
        unimplemented!()
    }

    fn bind(&self, mut bindings: Vec<(Pattern, Exp)>, exp: Exp) -> Exp {
        if let Some((pat, binding)) = bindings.pop() {
            let env = self.fun_env.module_env.env;
            let body = self.bind(bindings, exp);
            let loc = env.get_node_loc(pat.node_id());
            let ty = env.get_node_type(body.node_id());
            let new_id = env.new_node(loc, ty);
            ExpData::Block(new_id, pat, Some(binding), body).into_exp()
        } else {
            exp
        }
    }
}

impl<'a> ExpRewriterFunctions for LambdaLifter<'a> {
    fn rewrite_exp(&mut self, exp: Exp) -> Exp {
        // Intercept descent and compute lambdas being exempted from lifting, currently
        // those passed as parameters to inline functions.
        if self.options.include_inline_functions {
            return exp;
        }
        if let ExpData::Call(_, Operation::MoveFunction(mid, fid), args) = exp.as_ref() {
            let env = self.fun_env.module_env.env;
            if env.get_function(mid.qualified(*fid)).is_inline() {
                for arg in args {
                    if matches!(arg.as_ref(), ExpData::Lambda(..)) {
                        self.exempted_lambdas.insert(arg.node_id());
                    }
                }
            }
        }
        self.rewrite_exp_descent(exp)
    }

    fn rewrite_enter_scope<'b>(&mut self, vars: impl Iterator<Item = &'b (NodeId, Symbol)>) {
        self.scopes
            .push(vars.map(|(_, sym)| sym).cloned().collect())
    }

    fn rewrite_exit_scope(&mut self) {
        let scope = self.scopes.pop().expect("stack balanced");
        // Remove all locals which are bound in this scope
        self.free_locals.retain(|name, _| !scope.contains(name));
    }

    fn rewrite_local_var(&mut self, id: NodeId, sym: Symbol) -> Option<Exp> {
        // duplicates are OK -- they are all the same local at different locations
        self.free_locals.insert(sym, id);
        None
    }

    fn rewrite_temporary(&mut self, id: NodeId, idx: TempIndex) -> Option<Exp> {
        // duplicates are OK -- they are all the same parameter at different locations
        self.free_params.insert(idx, id);
        None
    }

    fn rewrite_lambda(&mut self, id: NodeId, pat: &Pattern, body: &Exp) -> Option<Exp> {
        if self.exempted_lambdas.contains(&id) {
            return None;
        }
        let env = self.fun_env.module_env.env;
        let mut params = vec![];
        let mut closure_args = vec![];
        // Add captured parameters
        for (param, node_id) in mem::take(&mut self.free_params) {
            let name = self.fun_env.get_local_name(param);
            let ty = env.get_node_type(node_id);
            let loc = env.get_node_loc(node_id);
            params.push(Parameter(name, ty.clone(), loc.clone()));
            let new_id = env.new_node(loc, ty);
            closure_args.push(ExpData::Temporary(new_id, param).into_exp())
        }
        // Add captured locals
        for (name, node_id) in mem::take(&mut self.free_locals) {
            let ty = env.get_node_type(node_id);
            let loc = env.get_node_loc(node_id);
            params.push(Parameter(name, ty.clone(), loc.clone()));
            let new_id = env.new_node(loc, ty);
            closure_args.push(ExpData::LocalVar(new_id, name).into_exp())
        }
        // Add lambda args. For dealing with patterns in lambdas (`|S{..}|e`) we need
        // to collect a list of bindings.
        let mut bindings = vec![]; //
        for arg in pat.clone().flatten() {
            let id = arg.node_id();
            let ty = env.get_node_type(id);
            let loc = env.get_node_loc(id);
            if let Pattern::Var(_, name) = arg {
                params.push(Parameter(name, ty, loc))
            } else {
                let name = self.gen_local_name();
                params.push(Parameter(name, ty.clone(), loc.clone()));
                let new_id = env.new_node(loc, ty);
                bindings.push((arg.clone(), ExpData::LocalVar(new_id, name).into_exp()))
            }
        }
        // Add new closure function
        let fun_name = self.gen_closure_function_name();
        let lambda_loc = env.get_node_loc(id).clone();
        let lambda_type = env.get_node_type(id);
        let result_type = if let Type::Fun(_, result_type) = &lambda_type {
            *result_type.clone()
        } else {
            Type::Error // type error reported
        };
        self.lifted.push(ClosureFunction {
            loc: lambda_loc.clone(),
            fun_id: FunId::new(fun_name),
            type_params: self.fun_env.get_type_parameters(),
            params,
            result_type,
            def: self.bind(bindings, body.clone()),
        });
        // Return closure expression
        let id = env.new_node(lambda_loc, lambda_type);
        Some(
            ExpData::Call(
                id,
                Operation::Closure(self.fun_env.module_env.get_id(), FunId::new(fun_name)),
                closure_args,
            )
            .into_exp(),
        )
    }
}