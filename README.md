# Telerust

## Introduction

The purpouse of this app is to offer a simple way to conect a cloud based bot, like Watson, to a Telegram chat. 

## How to run

### 1. Local
Firts create the `.env` file according to [env.sample](./env.sample) and type your credentials there. Now you just need to compile the code with `cargo build --release` command and then execute the binary with `./target/release/telerust`.

At this repo is provided the [Dockerfile](./Dockerfile) in case you want to run with Docker or deploy the app in some sort of container.

### 2. Docker

The simplest way is to build the image and store it in a registry, and then on the deploy paltform add the environment variables.

```bash
$ docker build -t image_name .
$ docker push image_name
```

However, you can create the `.env` file according to [env.sample](./env.sample) and add your credentials there, then uncomment the `COPY .env .` comamnd on the [Dockerfile](./Dockerfile). Build the image, store it in a registry service and deploy the docker image in the service of your preference.

There is another option, you can insert the environment varibales directly on the Dockerfile as shown below.
```dockerfile
ENV RUST_LOG=info

ENV TELEGRAM_TOKEN=YOUR_TELEGRAM_TOKEN
ENV WATSON_URL=YOUR_WATSON_URL
ENV WATSON_APIKEY=YOUR_WATSON_APIKEY
ENV WATSON_ASSISTANT_ID=YOUR_WATSON_ASSISTANT_ID
```

_Obs: It is highly recommended to avoid add the local environment variables in a file or inside the Dockerfile if you will store the docker image in a public registry as Docker Hub, you crendential would be exposed. Many deploy platforms have a secction where you can insert enviroment variables and this is safest option to proceed wit you choose to do it this way._

### 3. Kubernetes

This is repository is ready to deploy the application in kubernetes. You need to build and push the image to a resgistry with the following commands.

```bash
$ docker build -t image_name .
$ docker push image_name
```

In the [kubernetes](./kubernetes/) directory, there is the [deployment.yaml](./kubernetes/deployment.yaml) file which you just need to change line 21 to you image and add your environment variables as requested from line 34 to 37, then connect to your cluster run the following command.

```bash
$ kubectl apply -f kubernetes/deployment.yaml
```

_Obs: In the `alpine` branch is provided the dockerfile in the alpine version to save memory and the same instructions are valid to this dockerfile._

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