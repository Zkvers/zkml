use std::{collections::HashMap, marker::PhantomData, rc::Rc, vec};

use halo2_proofs::{
  circuit::{AssignedCell, Layouter},
  halo2curves::FieldExt,
  plonk::{ConstraintSystem, Error},
};
use ndarray::{Array, IxDyn};

use crate::gadgets::{
  gadget::{Gadget, GadgetConfig},
  sub_pairs::SubPairsChip,
};

use super::{
  super::layer::{Layer, LayerConfig, LayerType},
  Arithmetic,
};

#[derive(Clone, Debug)]
pub struct SubChip<F: FieldExt> {
  _marker: PhantomData<F>,
}

impl<F: FieldExt> SubChip<F> {
  pub fn construct() -> Self {
    Self {
      _marker: PhantomData,
    }
  }

  pub fn configure(_meta: &mut ConstraintSystem<F>, layer_params: Vec<i64>) -> LayerConfig {
    LayerConfig {
      layer_type: LayerType::Sub,
      layer_params,
      inp_shapes: vec![], // FIXME
      out_shapes: vec![],
    }
  }
}

impl<F: FieldExt> Arithmetic<F> for SubChip<F> {
  fn gadget_forward(
    &self,
    mut layouter: impl Layouter<F>,
    vec_inputs: &Vec<Vec<&AssignedCell<F, F>>>,
    constants: &Vec<AssignedCell<F, F>>,
    gadget_config: Rc<GadgetConfig>,
  ) -> Result<Vec<AssignedCell<F, F>>, Error> {
    let sub_pairs_chip = SubPairsChip::<F>::construct(gadget_config);
    let out = sub_pairs_chip.forward(layouter.namespace(|| "sub chip"), &vec_inputs, &constants)?;
    Ok(out)
  }
}

impl<F: FieldExt> Layer<F> for SubChip<F> {
  fn forward(
    &self,
    mut layouter: impl Layouter<F>,
    tensors: &Vec<Array<AssignedCell<F, F>, IxDyn>>,
    constants: &HashMap<i64, AssignedCell<F, F>>,
    gadget_config: Rc<GadgetConfig>,
  ) -> Result<Vec<Array<AssignedCell<F, F>, IxDyn>>, Error> {
    let (out, out_shape) = self.arithmetic_forward(
      layouter.namespace(|| ""),
      tensors,
      constants,
      gadget_config.clone(),
    )?;
    let out = Array::from_shape_vec(IxDyn(out_shape.as_slice()), out).unwrap();

    Ok(vec![out])
  }
}