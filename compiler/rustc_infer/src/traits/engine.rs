use crate::infer::InferCtxt;
use crate::traits::Obligation;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, Ty, Upcast};

use super::FulfillmentError;
use super::{ObligationCause, PredicateObligation};

pub trait TraitEngine<'tcx>: 'tcx {
    /// Requires that `ty` must implement the trait with `def_id` in
    /// the given environment. This trait must not have any type
    /// parameters (except for `Self`).
    fn register_bound(
        &mut self,
        infcx: &InferCtxt<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        ty: Ty<'tcx>,
        def_id: DefId,
        cause: ObligationCause<'tcx>,
    ) {
        let trait_ref = ty::TraitRef::new(infcx.tcx, def_id, [ty]);
        self.register_predicate_obligation(
            infcx,
            Obligation {
                cause,
                recursion_depth: 0,
                param_env,
                predicate: trait_ref.upcast(infcx.tcx),
            },
        );
    }

    fn register_predicate_obligation(
        &mut self,
        infcx: &InferCtxt<'tcx>,
        obligation: PredicateObligation<'tcx>,
    );

    fn register_predicate_obligations(
        &mut self,
        infcx: &InferCtxt<'tcx>,
        obligations: Vec<PredicateObligation<'tcx>>,
    ) {
        for obligation in obligations {
            self.register_predicate_obligation(infcx, obligation);
        }
    }

    #[must_use]
    fn select_where_possible(&mut self, infcx: &InferCtxt<'tcx>) -> Vec<FulfillmentError<'tcx>>;

    fn collect_remaining_errors(&mut self, infcx: &InferCtxt<'tcx>) -> Vec<FulfillmentError<'tcx>>;

    #[must_use]
    fn select_all_or_error(&mut self, infcx: &InferCtxt<'tcx>) -> Vec<FulfillmentError<'tcx>> {
        let errors = self.select_where_possible(infcx);
        if !errors.is_empty() {
            return errors;
        }

        self.collect_remaining_errors(infcx)
    }

    fn pending_obligations(&self) -> Vec<PredicateObligation<'tcx>>;

    /// Among all pending obligations, collect those are stalled on a inference variable which has
    /// changed since the last call to `select_where_possible`. Those obligations are marked as
    /// successful and returned.
    fn drain_unstalled_obligations(
        &mut self,
        infcx: &InferCtxt<'tcx>,
    ) -> Vec<PredicateObligation<'tcx>>;
}
