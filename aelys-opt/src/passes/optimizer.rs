use super::{
    ConstantFolder, DeadCodeEliminator, FunctionInliner, GlobalConstantPropagator,
    LocalConstantPropagator, OptimizationLevel, OptimizationPass, OptimizationStats,
    UnusedVarEliminator,
};
use aelys_sema::TypedProgram;

pub struct Optimizer {
    level: OptimizationLevel,
    passes: Vec<Box<dyn OptimizationPass>>,
    total_stats: OptimizationStats,
}

impl Optimizer {
    pub fn new(level: OptimizationLevel) -> Self {
        let mut passes: Vec<Box<dyn OptimizationPass>> = Vec::new();

        match level {
            OptimizationLevel::None => {}
            OptimizationLevel::Basic => {
                // conservative inlining first, then fold the results
                passes.push(Box::new(FunctionInliner::new(level)));
                passes.push(Box::new(LocalConstantPropagator::new()));
                passes.push(Box::new(ConstantFolder::new()));
            }
            OptimizationLevel::Standard => {
                passes.push(Box::new(FunctionInliner::new(level)));
                passes.push(Box::new(GlobalConstantPropagator::new()));
                passes.push(Box::new(LocalConstantPropagator::new()));
                passes.push(Box::new(ConstantFolder::new()));
                passes.push(Box::new(DeadCodeEliminator::new()));
                passes.push(Box::new(UnusedVarEliminator::new()));
                // second pass to catch opportunities created by earlier opts
                passes.push(Box::new(LocalConstantPropagator::new()));
                passes.push(Box::new(ConstantFolder::new()));
            }
            OptimizationLevel::Aggressive => {
                // aggressive inlining with higher bloat tolerance
                passes.push(Box::new(FunctionInliner::new(level)));
                passes.push(Box::new(GlobalConstantPropagator::new()));
                passes.push(Box::new(LocalConstantPropagator::new()));
                passes.push(Box::new(ConstantFolder::new()));
                passes.push(Box::new(DeadCodeEliminator::new()));
                passes.push(Box::new(UnusedVarEliminator::new()));
                // extra iteration for aggressive mode
                passes.push(Box::new(LocalConstantPropagator::new()));
                passes.push(Box::new(ConstantFolder::new()));
                passes.push(Box::new(DeadCodeEliminator::new()));
            }
        }

        Self { level, passes, total_stats: OptimizationStats::new() }
    }

    pub fn level(&self) -> OptimizationLevel { self.level }

    pub fn optimize(&mut self, mut program: TypedProgram) -> TypedProgram {
        for pass in &mut self.passes {
            self.total_stats.merge(&pass.run(&mut program));
        }
        program
    }

    pub fn stats(&self) -> &OptimizationStats { &self.total_stats }
}

impl Default for Optimizer {
    fn default() -> Self { Self::new(OptimizationLevel::Standard) }
}
