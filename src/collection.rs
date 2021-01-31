use uuid::Uuid;
use juniper::{Arguments, DefaultScalarValue, ExecutionResult, Executor, GraphQLType, GraphQLValue};

pub struct Entry {
    content: String,
    type_name: String,
    id: Uuid,
}

impl GraphQLValue for Entry {
    type Context = ();

    type TypeInfo = String;

    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        Self::name(info)
    }

    fn resolve_field(
        &self,
        _info: &String,
        field_name: &str,
        _args: &Arguments,
        executor: &Executor<()>,
    ) -> ExecutionResult
    {
        match field_name {
            "id" => executor.resolve_with_ctx(&(), &self.id),
            "title" => executor.resolve_with_ctx(&(), "Lorem Ipsum"),
            "content" => executor.resolve_with_ctx(&(), "Dolor sit amet"),
            _ => panic!("invalid field name"),
        }
    }
}

impl GraphQLType for Entry {
    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(info)
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut juniper::Registry<'r, DefaultScalarValue>) -> juniper::meta::MetaType<'r, DefaultScalarValue>
    where
        DefaultScalarValue: 'r
    {
        match info.as_str() {
            "blog_post" => {
                let fields = &[
                    registry.field::<String>("id", &()),
                    registry.field::<String>("title", &()),
                    registry.field::<String>("content", &()),
                ];
                registry.build_object_type::<Self>(info, fields).into_meta()
            },
            "note" => {
                let fields = &[
                    registry.field::<String>("text", &()),
                    registry.field::<String>("date", &()),
                ];
                registry.build_object_type::<Self>(info, fields).into_meta()
            },
            _ => panic!(),
        }
    }

}