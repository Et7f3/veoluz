type config = {
  .
  "lights":
    array({
      .
      "brightness": float,
      "kind": {
        .
        "Point": {
          .
          "offset": float,
          "origin": (float, float),
          "t0": float,
          "t1": float,
        },
      },
    }),
  "walls":
    array({
      .
      "kind": {
        .
        "Line":
          option({
            .
            "a": (float, float),
            "b": (float, float),
          }),
        "Parabola":
          option({
            .
            "a": float,
            "left": float,
            "right": float,
            "transform": {
              .
              "rotation": (float, float),
              "translation": (float, float),
            },
          }),
        "Circle":
          option(({. "radius": float}, (float, float), float, float)),
        // TODO circle
      },
      "properties": {
        .
        "absorb": float,
        "reflect": float,
        "roughness": float,
        "refraction": float,
      },
      "hide": bool,
    }),
  "light_formation": {
    .
    "Single": option(Js.null(unit)),
    "Line": option((int, float)),
    "Circle": option((int, float, bool)),
  },
  "transform": {
    .
    "rotational_symmetry": int,
    "reflection": bool,
  },
  "rendering": {
    .
    "center": (float, float),
    "coloration": {
      .
      "HueRange":
        option({
          .
          "start": float,
          "end": float,
          "saturation": float,
          "lightness": float,
        }),
      "Rgb":
        option({
          .
          "background": (float, float, float),
          "highlight": (float, float, float),
        }),
    },
    "exposure": {
      .
      "curve": string,
      "limits": Js.null((float, float)),
    },
    "height": int,
    "width": int,
    "zoom": float,
  },
};

type handle = {
  .
  "Handle": option(int),
  "Move": option((float, float)),
};

type ui = {
  .
  "show_lasers": bool,
  "selection":
    Js.null({
      .
      "Wall": option((int, Js.null((handle, (float, float))))),
      "Light": option((int, bool)),
      "Adding": option(string),
      // TODO fill in
      "Multiple":
        option(
          (
            array(int),
            Js.null((array((float, float)), (float, float))),
          ),
        ),
    }),
  "mouse_over": bool,
  "hovered": Js.null((int, handle)),
  "last_mouse_pos": (float, float),
};

let selectWall = (idx: int) => {
  "Wall": Some((idx, Js.null)),
  "Light": None,
  "Adding": None,
  "Multiple": None,
};

let selectLight = (idx: int) => {
  "Wall": None,
  "Light": Some((idx, false)),
  "Adding": None,
  "Multiple": None,
};

let blankUi: ui = {
  "show_lasers": false,
  "selection": Js.null,
  "mouse_over": false,
  "hovered": Js.null,
  "last_mouse_pos": (0.0, 0.0),
};

type wasm = {
  .
  "initial": [@bs.meth] (unit => config),
  "setup": [@bs.meth] ((config, (config, ui) => unit) => unit),
  "run": [@bs.meth] (unit => unit),
  "save": [@bs.meth] (unit => config),
  "restore": [@bs.meth] (config => config),
  "update": [@bs.meth] ((config, bool) => unit),
  "update_ui": [@bs.meth] (ui => unit),
  "blank_config": [@bs.meth] (unit => config),
  "show_hist": [@bs.meth] (Web.canvas => unit),
  "hide_hist": [@bs.meth] (unit => unit),
  "hover_wall": [@bs.meth] (int => unit),
  "show_ui": [@bs.meth] (unit => unit),
  "hide_ui": [@bs.meth] (unit => unit),
  "undo": [@bs.meth] (unit => unit),
  "test_run": [@bs.meth] (Web.canvas => unit),
  "redo": [@bs.meth] (unit => unit),
  "upvert": [@bs.meth] (config => Js.nullable(config)),
  "parse_url_config": [@bs.meth] (string => Js.nullable(config)),
  "serialize_url_config": [@bs.meth] (config => string),
};

let wasm: Js.Promise.t(wasm) = [%bs.raw {|import('../pkg/zenphoton')|}];

let withModule = fn =>
  wasm
  |> Js.Promise.then_(wasm => {
       fn(wasm);
       Js.Promise.resolve();
     })
  |> ignore;