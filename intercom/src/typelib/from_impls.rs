use super::*;

impl TypeLib
{
    pub fn from_comrc(lib: &ComRc<dyn IIntercomTypeLib>) -> Result<TypeLib, TypeLibError>
    {
        let mut types = vec![];
        for t in 0..lib.get_type_count()? {
            let ty = lib.get_type(t)?;

            types.push(match ty.get_kind()? {
                TypeInfoKind::CoClass => {
                    let cls = CoClass::from_comrc(&ComItf::query_interface(&ty)?)?;
                    TypeInfo::Class(ComBox::new(cls))
                }
                TypeInfoKind::Interface => {
                    let itf = Interface::from_comrc(&ComItf::query_interface(&ty)?)?;
                    TypeInfo::Interface(ComBox::new(itf))
                }
                TypeInfoKind::Struct => {
                    let stru = Struct::from_comrc(&ComItf::query_interface(&ty)?)?;
                    TypeInfo::Struct(ComBox::new(stru))
                }
            });
        }

        let (name, libid, version) = lib.get_info()?;
        Ok(TypeLib {
            name: name.into(),
            libid,
            version: version.into(),
            types,
        })
    }
}

impl CoClass
{
    pub fn from_comrc(ti: &ComRc<dyn IIntercomCoClass>) -> Result<CoClass, TypeLibError>
    {
        let mut interfaces = vec![];
        for i in 0..ti.get_interface_count()? {
            let (name, iid_automation) = ti.get_interface_ref(i, TypeSystemName::Automation)?;
            let (_, iid_raw) = ti.get_interface_ref(i, TypeSystemName::Raw)?;

            interfaces.push(InterfaceRef {
                name: name.into(),
                iid_automation,
                iid_raw,
            });
        }

        Ok(CoClass {
            name: ti.get_name()?.into(),
            clsid: ti.get_clsid()?,
            interfaces,
        })
    }
}

impl Interface
{
    pub fn from_comrc(ti: &ComRc<dyn IIntercomInterface>) -> Result<Interface, TypeLibError>
    {
        let mut variants = vec![];
        for v in 0..ti.get_variant_count()? {
            variants.push(ComBox::new(InterfaceVariant::from_comrc(
                &ti.get_variant(v)?,
            )?));
        }

        Ok(Interface {
            name: ti.get_name()?.into(),
            options: ti.get_options()?,
            variants,
        })
    }
}

impl InterfaceVariant
{
    pub fn from_comrc(
        ti: &ComRc<dyn IIntercomInterfaceVariant>,
    ) -> Result<InterfaceVariant, TypeLibError>
    {
        let mut methods = vec![];
        for m in 0..ti.get_method_count()? {
            methods.push(ComBox::new(Method::from_comrc(&ti.get_method(m)?)?));
        }

        Ok(InterfaceVariant {
            ts: ti.get_type_system()?,
            iid: ti.get_iid()?,
            methods,
        })
    }
}

impl Method
{
    pub fn from_comrc(ti: &ComRc<dyn IIntercomMethod>) -> Result<Method, TypeLibError>
    {
        let mut parameters = vec![];
        for p in 0..ti.get_parameter_count()? {
            let (name, ty, indirection_level, direction) = ti.get_parameter(p)?;
            parameters.push(Arg {
                name: name.into(),
                ty: ty.into(),
                indirection_level,
                direction,
            });
        }

        let (return_ty, return_indirection_level) = ti.get_return_type()?;
        Ok(Method {
            name: ti.get_name()?.into(),
            return_type: Arg {
                name: "".into(),
                ty: return_ty.into(),
                indirection_level: return_indirection_level,
                direction: Direction::Return,
            },
            parameters,
        })
    }
}

impl Struct
{
    pub fn from_comrc(ti: &ComRc<dyn IIntercomStruct>) -> Result<Self, TypeLibError>
    {
        let mut variants = vec![];
        for v in 0..ti.get_variant_count()? {
            variants.push(ComBox::new(StructVariant::from_comrc(&ti.get_variant(v)?)?));
        }
        Ok(Struct {
            name: ti.get_name()?.into(),
            variants,
        })
    }
}

impl StructVariant
{
    pub fn from_comrc(ti: &ComRc<dyn IIntercomStructVariant>) -> Result<Self, TypeLibError>
    {
        let mut fields = vec![];
        for f in 0..ti.get_field_count()? {
            let (name, ty, indirection_level, direction) = ti.get_field(f)?;
            fields.push(Arg {
                name: name.into(),
                ty: ty.into(),
                indirection_level,
                direction,
            });
        }
        Ok(StructVariant {
            ts: ti.get_type_system()?,
            fields,
        })
    }
}
