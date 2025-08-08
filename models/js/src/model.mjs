// import {
//   Directions,
//   ModelSpec,
//   Port,
//   PortEvent,
//   Relation
// } from './interfaces/taho-models-model'
// import { Ports } from './interfaces/taho-models-port'
// import { Subscription, Topic } from './interfaces/taho-models-subscriber'
// import { CrudApi } from './interfaces/taho-models-crud'
// export * as modelData from './interfaces/taho-models-model-data

//import { tahoModelsPublisher } from "./interfaces/taho-models-publisher"

//import { TextEncoder, TextDecoder } from "node:util"

const Relation = {
  name: 'string',
  domain: 'string',
  relatedModel: 'string',
  foreignKey: 'string',
  localOnly: true,
  keepRemote: false,
  desc: 'string'
}

const PortEvent = {
  name: 'string',
  wfid: 'string',
  state: 'string'
}

const PortItem = {
  name: 'string',
  adapter: 'string',
  direction: 'Inbound',
  consumes: [PortEvent],
  produces: [],
  callback: 'string',
  enabled: true,
  timeout: 0,
  retry: 0,
  interval: 0,
  circuitBreaker: false,
  retest: 0,
  undo: 'string'
}

const Spec = {
  name: 'model42',
  domain: 'domain2',
  endpoint: 'model42s',
  ports: [PortItem],
  relations: [Relation],
  datasource: 'redis_native'
}

export const modelData = {}

export const model = {
  getSpec () {
    return {
      name: 'model42',
      domain: 'domain2',
      endpoint: 'model42s',
      ports: [],
      relations: [],
      datasource: 'redis_cache'
    }
  }
}

const myPorts = {
  port1 (data) {

  },

  port2 (data) {
    console.log('\n 🚀 port2 was invoked 🌎\n')
    return data
  },

  fibonacci (bytes) {
    // const decoder = new TextDecoder();
    // const string1 = decoder.decode(bytes);
    // console.log(string1); // Output: "Hello"
    // const fib = JSON.parse(string1)
    // const val = fibonacci(fib.nth)
    // fib.result = val
    // const encoder = new TextEncoder();
    // const out = encoder.encode(JSON.stringify(fib))
    // return out
  }
}

function fibonacci(n) {
  if (n <= 1) {
    return n;
  } else {
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
}



class Ports {
  constructor(){
   
  }      

  invokePort (name, data) {
    let myPorts = Map([
      ['port1', myPorts.port1],
      ['port2', myPorts.port2]
    ])
    const port = myPorts.get(name)
    if (port) return port(data)
    return new Uint8Array(new ArrayBuffer())
  }

}

export const port = {
  Ports
}

class Subscription  {
  constructor(){}
  subscribe () {
    return ['*']
  }

  callback (topic, message) {
    console.log(`callback invoked ${topic} ${message}`)
  }
}

export const subscriber = {
  Subscription
}

class Crud {
  constructor(value) {

  }
  create (query, data) {
    return data
  }

  read (query, data) {
    return data
  }

  update (query, data) {
    return data
  }

  delete (query) {
    return
  }
}

export const crud = {
  CrudApi: Crud
}
