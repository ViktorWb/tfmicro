use bindgen::{
    callbacks::{
        DeriveInfo, DeriveTrait, EnumVariantCustomBehavior, EnumVariantValue,
        FieldInfo, ImplementsTrait, IntKind, ItemInfo, MacroParsingBehavior,
    },
    FieldVisibilityKind,
};

#[derive(Debug)]
pub struct CustomCargoCallbacks(pub bindgen::CargoCallbacks);

impl bindgen::callbacks::ParseCallbacks for CustomCargoCallbacks {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.0.will_parse_macro(name)
    }

    fn generated_name_override(
        &self,
        item_info: ItemInfo<'_>,
    ) -> Option<String> {
        self.0.generated_name_override(item_info)
    }

    fn generated_link_name_override(
        &self,
        item_info: ItemInfo<'_>,
    ) -> Option<String> {
        self.0.generated_link_name_override(item_info)
    }

    fn int_macro(&self, name: &str, value: i64) -> Option<IntKind> {
        self.0.int_macro(name, value)
    }

    fn str_macro(&self, name: &str, value: &[u8]) {
        self.0.str_macro(name, value)
    }

    fn func_macro(&self, name: &str, value: &[&[u8]]) {
        self.0.func_macro(name, value)
    }

    fn enum_variant_behavior(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        self.0.enum_variant_behavior(
            enum_name,
            original_variant_name,
            variant_value,
        )
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: EnumVariantValue,
    ) -> Option<String> {
        self.0.enum_variant_name(
            enum_name,
            original_variant_name,
            variant_value,
        )
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        self.0.item_name(original_item_name)
    }

    fn header_file(&self, filename: &str) {
        self.0.header_file(filename)
    }

    fn include_file(&self, filename: &str) {
        self.0.include_file(filename)
    }

    fn read_env_var(&self, key: &str) {
        self.0.read_env_var(key)
    }

    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        self.0.blocklisted_type_implements_trait(name, derive_trait)
    }

    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        self.0.add_derives(info)
    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        // Ignore doc tests
        // https://github.com/rust-lang/rust-bindgen/issues/1313
        Some(format!("````ignore\n{comment}\n````"))
    }

    fn field_visibility(
        &self,
        info: FieldInfo<'_>,
    ) -> Option<FieldVisibilityKind> {
        self.0.field_visibility(info)
    }
}
