use super::{Function, Parameterised, VFunction, QFunction};
use geometry::Space;
use std::marker::PhantomData;


pub struct VFunctionGroup<S: Space, V: VFunction<S>>(Vec<V>, PhantomData<S>);

impl<S: Space, V: VFunction<S>> VFunctionGroup<S, V>
{
    pub fn new(functions: Vec<V>) -> Self {
        VFunctionGroup(functions, PhantomData)
    }
}

impl<S: Space, V: VFunction<S>> Function<S::Repr, Vec<f64>> for VFunctionGroup<S, V>
{
    fn evaluate(&self, state: &S::Repr) -> Vec<f64> {
        self.0.iter().map(|f| f.evaluate(state)).collect()
    }
}

impl<S: Space, V: VFunction<S>> Parameterised<S::Repr, Vec<f64>> for VFunctionGroup<S, V>
{
    fn update(&mut self, state: &S::Repr, mut errors: Vec<f64>) {
        let mut index = 0;

        for e in errors.drain(..) {
            self.0[index].update(state, e);
            index += 1;
        }
    }
}

impl<S: Space, V: VFunction<S>> QFunction<S> for VFunctionGroup<S, V>
{
    fn evaluate_action(&self, state: &S::Repr, action: usize) -> f64 {
        self.0[action].evaluate(state)
    }

    fn update_action(&mut self, state: &S::Repr, action: usize, error: f64) {
        self.0[action].update(state, error);
    }
}


#[cfg(test)]
mod tests {
    use super::{VFunctionGroup, Function, Parameterised, VFunction, QFunction};
    use geometry::RegularSpace;
    use geometry::dimensions::Continuous;


    pub struct Mock(f64);

    impl Function<Vec<f64>, f64> for Mock {
        fn evaluate(&self, _: &Vec<f64>) -> f64 { self.0 }
    }

    impl Parameterised<Vec<f64>, f64> for Mock {
        fn update(&mut self, _: &Vec<f64>, e: f64) { self.0 = e; }
    }

    impl VFunction<RegularSpace<Continuous>> for Mock {}


    #[test]
    fn test_function() {
        let fg = VFunctionGroup::new(vec![Mock(0.0), Mock(1.0), Mock(2.0)]);

        assert_eq!(fg.evaluate(&vec![]), vec![0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_parameterised() {
        let mut fg = VFunctionGroup::new(vec![Mock(0.0), Mock(1.0), Mock(2.0)]);

        fg.update(&vec![], vec![4.0, -1.0, 8.0]);

        assert_eq!(fg.evaluate(&vec![]), vec![4.0, -1.0, 8.0]);
    }

    #[test]
    fn test_qfunction() {
        let fg = VFunctionGroup::new(vec![Mock(0.0), Mock(1.0), Mock(2.0)]);

        assert_eq!(fg.evaluate_action(&vec![], 0), 0.0);
        assert_eq!(fg.evaluate_action(&vec![], 1), 1.0);
        assert_eq!(fg.evaluate_action(&vec![], 2), 2.0);
    }

    #[test]
    fn test_boxed_vfuncs() {
        let mut fg = VFunctionGroup::new(vec![
            Box::new(Mock(0.0)),
            Box::new(Mock(1.0)),
            Box::new(Mock(2.0))
        ]);

        assert_eq!(fg.evaluate(&vec![]), vec![0.0, 1.0, 2.0]);

        assert_eq!(fg.evaluate_action(&vec![], 0), 0.0);
        assert_eq!(fg.evaluate_action(&vec![], 1), 1.0);
        assert_eq!(fg.evaluate_action(&vec![], 2), 2.0);

        fg.update(&vec![], vec![4.0, -1.0, 8.0]);

        assert_eq!(fg.evaluate(&vec![]), vec![4.0, -1.0, 8.0]);
    }
}