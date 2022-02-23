#!/bin/sh
# jq is required : npm i jq.node --global
curl --silent https://raw.githubusercontent.com/sindresorhus/cli-spinners/master/spinners.json | jqn -r lodash --color=false 'toPairs | map(([k, v]) => ({...v, name: lodash.upperFirst(k)}))' > src/spinners.json


jqn 'pluck("name") | thru(x => `use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, EnumIter, Display, EnumString)]
pub enum SpinnerNames{${x.join(",\n")}}`)' < src/spinners.json  | rustfmt > src/utils/spinner_names.rs

jqn 'thru(ipt => `use std::collections::HashMap;
use crate::utils::spinner_data::SpinnerData;
use lazy_static::lazy_static;
use maplit::{self, hashmap};
lazy_static! {
  pub static ref SPINNERS: HashMap<String, SpinnerData> = {
    hashmap! {
      ${ipt.map(spinner => "\""+spinner.name+"\".into() => SpinnerData {frames: vec!" + JSON.stringify(spinner.frames, null, 2) + ", interval: "+ spinner.interval + "}").join(",\n")}
    }
  }
}
`)' < src/spinners.json | rustfmt  > src/utils/spinners_data.rs

rm src/spinners.json
