// Generated by BUCKLESCRIPT VERSION 5.0.4, PLEASE EDIT WITH CARE
'use strict';

var Block = require("bs-platform/lib/js/block.js");
var Curry = require("bs-platform/lib/js/curry.js");
var Js_dict = require("bs-platform/lib/js/js_dict.js");
var Js_json = require("bs-platform/lib/js/js_json.js");
var Belt_List = require("bs-platform/lib/js/belt_List.js");
var Belt_Array = require("bs-platform/lib/js/belt_Array.js");
var Caml_option = require("bs-platform/lib/js/caml_option.js");
var TypeHelpers = require("./TypeHelpers.bs.js");

var Types1 = /* module */[];

var schemaPropertyName = "$schemaVersion";

var deserialize_Belt_SetString____t = TypeHelpers.deserialize_Belt_SetString____t;

var serialize_Belt_SetString____t = TypeHelpers.serialize_Belt_SetString____t;

function serialize_Types____scene(record) {
  var array = record[/* children */2];
  var transformer = function (prim) {
    return prim;
  };
  return Js_dict.fromArray(/* array */[
              /* tuple */[
                "id",
                record[/* id */0]
              ],
              /* tuple */[
                "tags",
                Curry._1(serialize_Belt_SetString____t, record[/* tags */1])
              ],
              /* tuple */[
                "children",
                Belt_Array.map(array, transformer)
              ]
            ]);
}

function deserialize_Types____scene(record) {
  var match = Js_json.classify(record);
  if (typeof match === "number" || match.tag !== 2) {
    return /* Error */Block.__(1, [/* :: */[
                "Expected an object",
                /* [] */0
              ]]);
  } else {
    var dict = match[0];
    var match$1 = Js_dict.get(dict, "children");
    if (match$1 !== undefined) {
      var transformer = function (string) {
        var match = Js_json.classify(string);
        if (typeof match === "number" || match.tag) {
          return /* Error */Block.__(1, [/* :: */[
                      "expected a string",
                      /* [] */0
                    ]]);
        } else {
          return /* Ok */Block.__(0, [match[0]]);
        }
      };
      var match$2 = Js_json.classify(Caml_option.valFromOption(match$1));
      var match$3;
      if (typeof match$2 === "number" || match$2.tag !== 3) {
        match$3 = /* Error */Block.__(1, [/* :: */[
              "expected an array",
              /* [] */0
            ]]);
      } else {
        var loop = function (_i, _collected, _items) {
          while(true) {
            var items = _items;
            var collected = _collected;
            var i = _i;
            if (items) {
              var match = transformer(items[0]);
              if (match.tag) {
                return /* Error */Block.__(1, [/* :: */[
                            "list element " + String(i),
                            match[0]
                          ]]);
              } else {
                _items = items[1];
                _collected = /* :: */[
                  match[0],
                  collected
                ];
                _i = i + 1 | 0;
                continue ;
              }
            } else {
              return /* Ok */Block.__(0, [Belt_List.reverse(collected)]);
            }
          };
        };
        var match$4 = loop(0, /* [] */0, Belt_List.fromArray(match$2[0]));
        match$3 = match$4.tag ? /* Error */Block.__(1, [match$4[0]]) : /* Ok */Block.__(0, [Belt_List.toArray(match$4[0])]);
      }
      if (match$3.tag) {
        return /* Error */Block.__(1, [/* :: */[
                    "attribute 'children'",
                    match$3[0]
                  ]]);
      } else {
        var attr_children = match$3[0];
        var match$5 = Js_dict.get(dict, "tags");
        if (match$5 !== undefined) {
          var match$6 = Curry._1(deserialize_Belt_SetString____t, Caml_option.valFromOption(match$5));
          if (match$6.tag) {
            return /* Error */Block.__(1, [/* :: */[
                        "attribute 'tags'",
                        match$6[0]
                      ]]);
          } else {
            var attr_tags = match$6[0];
            var match$7 = Js_dict.get(dict, "id");
            if (match$7 !== undefined) {
              var match$8 = Js_json.classify(Caml_option.valFromOption(match$7));
              var match$9;
              match$9 = typeof match$8 === "number" || match$8.tag ? /* Error */Block.__(1, [/* :: */[
                      "expected a string",
                      /* [] */0
                    ]]) : /* Ok */Block.__(0, [match$8[0]]);
              if (match$9.tag) {
                return /* Error */Block.__(1, [/* :: */[
                            "attribute 'id'",
                            match$9[0]
                          ]]);
              } else {
                var attr_id = match$9[0];
                return /* Ok */Block.__(0, [/* record */[
                            /* id */attr_id,
                            /* tags */attr_tags,
                            /* children */attr_children
                          ]]);
              }
            } else {
              return /* Error */Block.__(1, [/* :: */[
                          "No attribute 'id'",
                          /* [] */0
                        ]]);
            }
          }
        } else {
          return /* Error */Block.__(1, [/* :: */[
                      "No attribute 'tags'",
                      /* [] */0
                    ]]);
        }
      }
    } else {
      return /* Error */Block.__(1, [/* :: */[
                  "No attribute 'children'",
                  /* [] */0
                ]]);
    }
  }
}

function deserialize_Types____directory(value) {
  var match = Js_json.classify(value);
  if (typeof match === "number" || match.tag !== 3) {
    return /* Error */Block.__(1, [/* :: */[
                "expected an array",
                /* [] */0
              ]]);
  } else {
    var loop = function (_i, _collected, _items) {
      while(true) {
        var items = _items;
        var collected = _collected;
        var i = _i;
        if (items) {
          var match = deserialize_Types____scene(items[0]);
          if (match.tag) {
            return /* Error */Block.__(1, [/* :: */[
                        "list element " + String(i),
                        match[0]
                      ]]);
          } else {
            _items = items[1];
            _collected = /* :: */[
              match[0],
              collected
            ];
            _i = i + 1 | 0;
            continue ;
          }
        } else {
          return /* Ok */Block.__(0, [Belt_List.reverse(collected)]);
        }
      };
    };
    var match$1 = loop(0, /* [] */0, Belt_List.fromArray(match[0]));
    if (match$1.tag) {
      return /* Error */Block.__(1, [match$1[0]]);
    } else {
      return /* Ok */Block.__(0, [Belt_List.toArray(match$1[0])]);
    }
  }
}

function serialize_Types____directory(value) {
  return Belt_Array.map(value, serialize_Types____scene);
}

var Version1 = /* module */[
  /* deserialize_Belt_SetString____t */deserialize_Belt_SetString____t,
  /* deserialize_Types____directory */deserialize_Types____directory,
  /* deserialize_Types____scene */deserialize_Types____scene,
  /* serialize_Belt_SetString____t */serialize_Belt_SetString____t,
  /* serialize_Types____directory */serialize_Types____directory,
  /* serialize_Types____scene */serialize_Types____scene
];

function parseVersion(json) {
  var match = Js_json.classify(json);
  if (typeof match === "number") {
    return /* Error */Block.__(1, ["Must have a schema version"]);
  } else {
    switch (match.tag | 0) {
      case 2 : 
          var match$1 = Js_dict.get(match[0], schemaPropertyName);
          if (match$1 !== undefined) {
            var match$2 = Js_json.classify(Caml_option.valFromOption(match$1));
            if (typeof match$2 === "number" || match$2.tag !== 1) {
              return /* Error */Block.__(1, ["Invalid $schemaVersion"]);
            } else {
              return /* Ok */Block.__(0, [/* tuple */[
                          match$2[0] | 0,
                          json
                        ]]);
            }
          } else {
            return /* Error */Block.__(1, ["No $schemaVersion present"]);
          }
      case 3 : 
          var match$3 = match[0];
          if (match$3.length !== 2) {
            return /* Error */Block.__(1, ["Must have a schema version"]);
          } else {
            var version = match$3[0];
            var payload = match$3[1];
            var match$4 = Js_json.classify(version);
            if (typeof match$4 === "number" || match$4.tag !== 1) {
              return /* Error */Block.__(1, ["Invalid wrapped version"]);
            } else {
              return /* Ok */Block.__(0, [/* tuple */[
                          match$4[0] | 0,
                          payload
                        ]]);
            }
          }
      default:
        return /* Error */Block.__(1, ["Must have a schema version"]);
    }
  }
}

function wrapWithVersion(version, payload) {
  var match = Js_json.classify(payload);
  if (typeof match === "number") {
    return /* array */[
            version,
            payload
          ];
  } else if (match.tag === 2) {
    var dict = match[0];
    dict[schemaPropertyName] = version;
    return dict;
  } else {
    return /* array */[
            version,
            payload
          ];
  }
}

function serializeDirectory(data) {
  return wrapWithVersion(1, serialize_Types____directory(data));
}

function deserializeDirectory(data) {
  var match = parseVersion(data);
  if (match.tag) {
    return /* Error */Block.__(1, [/* :: */[
                match[0],
                /* [] */0
              ]]);
  } else {
    var match$1 = match[0];
    var version = match$1[0];
    if (version !== 1) {
      return /* Error */Block.__(1, [/* :: */[
                  "Unexpected version " + String(version),
                  /* [] */0
                ]]);
    } else {
      var match$2 = deserialize_Types____directory(match$1[1]);
      if (match$2.tag) {
        return /* Error */Block.__(1, [match$2[0]]);
      } else {
        return /* Ok */Block.__(0, [match$2[0]]);
      }
    }
  }
}

var Directory = /* module */[
  /* serialize */serializeDirectory,
  /* deserialize */deserializeDirectory
];

var Modules = /* module */[/* Directory */Directory];

var currentVersion = 1;

var Current = 0;

exports.Types1 = Types1;
exports.currentVersion = currentVersion;
exports.schemaPropertyName = schemaPropertyName;
exports.Version1 = Version1;
exports.Current = Current;
exports.parseVersion = parseVersion;
exports.wrapWithVersion = wrapWithVersion;
exports.serializeDirectory = serializeDirectory;
exports.deserializeDirectory = deserializeDirectory;
exports.Modules = Modules;
/* No side effect */