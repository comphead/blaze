// Copyright 2022 The Blaze Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use datafusion::common::{DataFusionError, Result};
use datafusion::logical_expr::ScalarFunctionImplementation;
use std::sync::Arc;

mod spark_check_overflow;
mod spark_get_json_object;
mod spark_make_array;
mod spark_make_decimal;
mod spark_murmur3_hash;
mod spark_null_if_zero;
mod spark_strings;
mod spark_unscaled_value;

pub fn create_spark_ext_function(name: &str) -> Result<ScalarFunctionImplementation> {
    Ok(match name {
        "Placeholder" => Arc::new(|_| panic!("placeholder() should never be called")),
        "NullIfZero" => Arc::new(spark_null_if_zero::spark_null_if_zero),
        "UnscaledValue" => Arc::new(spark_unscaled_value::spark_unscaled_value),
        "MakeDecimal" => Arc::new(spark_make_decimal::spark_make_decimal),
        "CheckOverflow" => Arc::new(spark_check_overflow::spark_check_overflow),
        "Murmur3Hash" => Arc::new(spark_murmur3_hash::spark_murmur3_hash),
        "GetJsonObject" => Arc::new(spark_get_json_object::spark_get_json_object),
        "GetParsedJsonObject" => Arc::new(spark_get_json_object::spark_get_parsed_json_object),
        "ParseJson" => Arc::new(spark_get_json_object::spark_parse_json),
        "MakeArray" => Arc::new(spark_make_array::array),
        "StringSpace" => Arc::new(spark_strings::string_space),
        "StringRepeat" => Arc::new(spark_strings::string_repeat),
        "StringSplit" => Arc::new(spark_strings::string_split),
        "StringConcat" => Arc::new(spark_strings::string_concat),
        "StringConcatWs" => Arc::new(spark_strings::string_concat_ws),
        "StringLower" => Arc::new(spark_strings::string_lower),
        "StringUpper" => Arc::new(spark_strings::string_upper),

        _ => Err(DataFusionError::NotImplemented(format!(
            "spark ext function not implemented: {}",
            name
        )))?,
    })
}
