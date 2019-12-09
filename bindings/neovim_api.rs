// Auto generated {{date}}
use std::io::Write;

use crate::neovim::*;
use crate::rpc::*;
use crate::callerror::{CallError, map_generic_error};

fn map_result<T: FromVal<Value>>(val: Value) -> T {
    T::from_val(val)
}

{% for etype in exttypes %}
#[derive(PartialEq, Clone, Debug)]
pub struct {{ etype.name }} {
    code_data: Value,
}

impl {{ etype.name }} {
    pub fn new(code_data: Value) -> {{ etype.name }} {
        {{ etype.name }} {
            code_data,
        }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    {% for f in functions if f.ext and f.name.startswith(etype.prefix) %}
    /// since: {{f.since}}
    pub async fn {{f.name|replace(etype.prefix, '')}}<W>(&self, neovim: &Neovim<W>, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, CallError>
    where W: Write + Send + 'static
    {
        neovim.call("{{f.name}}",
                          call_args![self.code_data.clone()
                          {% if f.parameters|count > 0 %}
                          , {{ f.parameters|map(attribute = "name")|join(", ") }}
                          {% endif %}
                          ])
                    .await
                    .map(map_result)
                    .map_err(map_generic_error)
    }
    {% endfor %}
}

{% endfor %}

{% for etype in exttypes %}
impl FromVal<Value> for {{ etype.name }} {
    fn from_val(val: Value) -> Self {
        {{ etype.name }}::new(val)
    }
}

impl <'a> IntoVal<Value> for &'a {{etype.name}} {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
{% endfor %}

impl<W> Requester<W>
where
      W: Write + Send + 'static,
{
    {% for f in functions if not f.ext %}
    pub async fn {{f.name|replace('nvim_', '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, CallError> {
        self.call("{{f.name}}",
                          call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .await
                    .map(map_result)
                    .map_err(map_generic_error)
    }

    {% endfor %}
}

impl<W> Neovim<W>
where
      W: Write + Send + 'static,
{
    {% for f in functions if not f.ext %}
    pub async fn {{f.name|replace('nvim_', '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, CallError> {
        self.call("{{f.name}}",
                          call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .await
                    .map(map_result)
                    .map_err(map_generic_error)
    }

    {% endfor %}
}
