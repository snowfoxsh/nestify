use crate::attributes::{Attribute, AttributeModifier, CompositeAttribute, FieldAttribute};

#[derive(Clone, Default)]
pub(crate) struct UnpackContext {
    inherited: Vec<Attribute>,
}

impl UnpackContext {
    pub fn modify_composite(&mut self, attributes: Vec<CompositeAttribute>) -> Vec<Attribute> {
        let mut freeze = self.inherited.clone();

        let current = attributes
            .into_iter()
            .filter_map(|ca| {
                // handle standard attribute
                let Some(modifier) = &ca.modifier else {
                    let a: Attribute = ca.into();

                    if freeze.contains(&a) {
                        panic!("already in the stack");
                        //todo: improve this error message
                        // already defined so do something
                    }

                    return Some(a);
                };

                // handle attribute with modifier
                match modifier {
                    AttributeModifier::Star(_) => {
                        let a: Attribute = ca.into();

                        // already defined so lets error
                        if freeze.contains(&a) {
                            panic!("already in the stack (*)")
                            // todo: improve error msg
                        }

                        // add attribute to the downstream stack
                        self.inherited.push(a.clone());

                        // return the attribute
                        Some(a)
                    }
                    AttributeModifier::Slash(_) => {
                        let a: Attribute = ca.into();

                        // already defined so lets error
                        if !freeze.contains(&a) {
                            panic!("not in the stack, dont need to remove (/)");
                        }

                        // remove from the future
                        self.inherited.retain(|attr| attr != &a);

                        // remove from freeze
                        freeze.retain(|attr| attr != &a);

                        // remove it from the current
                        None
                    }
                    AttributeModifier::Minus(_) => {
                        let a: Attribute = ca.into();

                        // not in the stack so cant remove, lets error
                        if !freeze.contains(&a) {
                            panic!("not in the stack, dont need to remove (-)");
                        }

                        // dont remove it from the future

                        // remove from freeze
                        freeze.retain(|attr| attr != &a);

                        // remove it from the current
                        None
                    }
                }
            })
            .collect();

        [freeze, current].concat()
    }

    pub(crate) fn filter_field_nested(field_attributes: Vec<FieldAttribute>) -> (Vec<Attribute>, Vec<CompositeAttribute>) {
        let mut field_applied_now = vec![];
        let mut composite = vec![];

        field_attributes.into_iter().for_each(|attr| {
            match attr {
                FieldAttribute::Field(fa) => field_applied_now.push(fa),
                FieldAttribute::Nested(na) => composite.push(na.into())
            }
        });

        (field_applied_now, composite)
    }
}
