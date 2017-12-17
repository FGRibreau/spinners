#!/bin/sh
# jq is required : npm i jq.node --global
curl --silent https://raw.githubusercontent.com/sindresorhus/cli-spinners/master/spinners.json | jqn -r lodash --color=false 'toPairs | map(([k, v]) => ({...v, name: lodash.upperFirst(k)}))' > src/spinners.json
jqn 'pluck("name") | thru(x => `\n#[derive(Debug, Clone, EnumString, EnumIter)]\npub enum SpinnerNames{${x.join(",\n")}}`)' < src/spinners.json  | rustfmt > src/spinner_names.rs
jqn 'thru(ipt => "use spinner_data::SpinnerData;\nlazy_static! {\npub static ref SPINNERS: Vec<SpinnerData> = {\nvec![" + ipt.map(spinner => `SpinnerData {name: ${JSON.stringify(spinner.name)}.into(), frames: vec!${JSON.stringify(spinner.frames, null, 2)}, interval: ${spinner.interval}}`).join(",\n")+"]};}")' < src/spinners.json | rustfmt  > src/spinners_data.rs
