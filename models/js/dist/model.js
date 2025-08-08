var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/model.mjs
var model_exports = {};
__export(model_exports, {
  crud: () => crud,
  model: () => model,
  modelData: () => modelData,
  port: () => port,
  subscriber: () => subscriber
});
module.exports = __toCommonJS(model_exports);
var modelData = {};
var model = {
  getSpec() {
    return {
      name: "model42",
      domain: "domain2",
      endpoint: "model42s",
      ports: [],
      relations: [],
      datasource: "redis_cache"
    };
  }
};
var Ports = class {
  constructor() {
  }
  invokePort(name, data) {
    let myPorts = Map([
      ["port1", myPorts.port1],
      ["port2", myPorts.port2]
    ]);
    const port2 = myPorts.get(name);
    if (port2) return port2(data);
    return new Uint8Array(new ArrayBuffer());
  }
};
var port = {
  Ports
};
var Subscription = class {
  constructor() {
  }
  subscribe() {
    return ["*"];
  }
  callback(topic, message) {
    console.log(`callback invoked ${topic} ${message}`);
  }
};
var subscriber = {
  Subscription
};
var Crud = class {
  constructor(value) {
  }
  create(query, data) {
    return data;
  }
  read(query, data) {
    return data;
  }
  update(query, data) {
    return data;
  }
  delete(query) {
    return;
  }
};
var crud = {
  CrudApi: Crud
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  crud,
  model,
  modelData,
  port,
  subscriber
});
