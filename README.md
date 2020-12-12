# Telerust

## What does this code do?

The purpouse of this app is to offer a simple way to conect a cloud based bot, like Watson, to a Telegram chat. 

## and how do I run it

Firts create the `.env` file according to [env.sample](./env.sample) and type your credentials there. Now you just need to compile the code with `cargo build --realease` commando and then execute the binary with `./target/release/telerust`.

At this repo is provided the [Dockerfile](./Dockerfile) in case you want to run with Docker or deploy the app in some sort of container .

## Questions

For any questions you may have please open an issue or contact the maintener by email. Feel free to submit your pull request or upgrades to this repo.

## Lincense

Copyright 2020 João Pedro Poloni Ponce

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.