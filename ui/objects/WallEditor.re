open Ui;

module Triangle = {
  [@react.component]
  let make = (~size, ~padding) => {
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width={string_of_int(padding + size + padding)}
      height={string_of_int(padding + size + padding)}>
      <path
        fill="white"
        stroke="black"
        d={Printf.sprintf(
          "M%d,%d L%d,%d L%d,%d z",
          padding,
          padding,
          padding,
          padding + size,
          padding + int_of_float(float_of_int(size) *. sqrt(3.0) /. 2.0),
          padding + size / 2,
        )}
      />
    </svg>;
  };
};

let dist = ((x0, y0), (x1, y1)) => {
  let dx = x1 -. x0;
  let dy = y1 -. y0;
  sqrt(dx *. dx +. dy *. dy);
};

let toLine = ((x0, y0), (x1, y1)) => {
  // y = mx + b
  let m = (y1 -. y0) /. (x1 -. x0);
  // y0 = m * x0 + b
  // b = y0 - m * x0
  // b = y1 - m * x1
  // y1 = m * x1 + b
  let b = y0 -. m *. x0;
  (m, b);
};

let lineIntersect = ((m0, b0), (m1, b1)) => {
  // y = m0 x + b0
  // y = m1 x + b1
  // m0 x + b0 = m1 x + b1
  // m0 x - m1 x = b1 - b0
  // x = (b1 - b0) / (m0 - m1)
  let x = (b1 -. b0) /. (m0 -. m1);
  let y = m0 *. x +. b0;
  (x, y);
};

let triWidth = size => size *. sqrt(3.0) /. 2.0;
let bottomPoint = size => (0.0, size);
let rightPoint = size => (triWidth(size), size /. 2.0);


let posTriangle = (~pos as (x, y), ~size) => {
  let ew = x /. triWidth(size);
  let farPoint =
    lineIntersect(
      toLine((x, y), bottomPoint(size)),
      // the top right line
      toLine((0.0, 0.0), rightPoint(size)),
    );
  let fullSize = dist(bottomPoint(size), farPoint);
  let partSize = dist((x, y), bottomPoint(size));

  (max(0.0, min(1.0, partSize /. fullSize)), max(0.0, min(1.0, ew)));
};

let trianglePos = ((se, ew), ~size) => {
  let www = triWidth(size);
  // let vertical = toLine((0.0, www *. ew), );
  let m = -. sqrt(3.0);
  let b = se *. size;
  // y = mx + b
  let x = www *. ew;
  let y = m *. x +. se *. size;
  // let pos = lineIntersect(vertical, fromBottomLeft);
  (x, y)
  // (ns *. size, ew *. size);
};

module TriangleEditor = {
  [@react.component]
  let make = (~ns, ~se, ~onChange) => {
    let node = React.useRef(Js.Nullable.null);
    let (_, onMouseDown) =
      ExposureControl.useDraggable(~onMove=((x, y)) => {
        switch (Js.Nullable.toOption(node->React.Ref.current)) {
          | None => ()
          | Some(node) =>
            let rect = Web.getBoundingClientRect(node);
            let x = float_of_int(x) -. rect##left -. 10.0;
            let y = float_of_int(y) -. rect##top -. 10.0;
            let (ns, se) =
              posTriangle(
                ~pos=(x, y),
                ~size=100.0,
              );
            Js.log3("Pos", (x, y), (ns, se));
            onChange((ns, se));
        }
      });

    let (x, y) = trianglePos((ns, se), ~size=100.0);
    let y = 100.0 -. y;

    <div
      ref={ReactDOMRe.Ref.domRef(node)}
      className=Css.(style([position(`relative)]))>
      <Triangle size=100 padding=10 />
      <div
        onMouseDown
        className=Css.(
          style([
            width(px(10)),
            height(px(10)),
            marginLeft(px(-5)),
            marginTop(px(-5)),
            position(`absolute),
            backgroundColor(red),
          ])
        )
        style={ReactDOMRe.Style.make(
          ~left=Js.Float.toString(x +. 10.0) ++ "px",
          ~top=Js.Float.toString(y +. 10.0) ++ "px",
          (),
        )}
      />
    </div>;
  };
};

module TriangleTester = {
  [@react.component]
  let make = () => {
    let ((ns, se), onChange) = Hooks.useState((0.0, 0.0));
    <div className=Css.(style([backgroundColor(white), padding(px(100))]))>
      <TriangleEditor ns se onChange />
    </div>;
  };
};

let wallType = kind =>
  if ([%js.deep kind["Line"]] != None) {
    "Line";
  } else if ([%js.deep kind["Parabola"]] != None) {
    "Parabola";
  } else {
    "Arc";
  };

[@react.component]
let make =
    (~wasm, ~selected, ~wall, ~index, ~onChange, ~onRemove, ~updateUi, ~ui) => {
  <div
    className=Css.(
      style(
        [
          // cursor(`pointer),
          padding2(~v=px(8), ~h=`zero),
          borderBottom(px(1), `solid, hex("ddd")),
          hover([backgroundColor(Colors.buttonHover)]),
        ]
        @ (
          selected
            ? [
              outline(px(2), `solid, Colors.accent),
              hover([outline(px(2), `solid, Colors.accent)]),
            ]
            : []
        ),
      )
    )
    onMouseOver={evt => wasm##hover_wall(index)}>
    <div
      className=Css.(
        style([
          display(`flex),
          paddingRight(px(8)),
          cursor(`pointer),
          // justifyContent(`spaceBetween),
          alignItems(`center),
        ])
      )
      onClick={evt =>
        if (selected) {
          updateUi([%js.deep ui["selection"].replace(Js.null)]);
        } else {
          wasm##set_active_wall(index);
        }
      }>
      {selected
         ? <IonIcons.ArrowDown fontSize="14px" />
         : <IonIcons.ArrowRight fontSize="14px" />}
      {Styles.spacer(4)}
      <div
        className=Css.(
          style([fontWeight(`medium), flex(1), fontSize(px(12))])
        )>
        {React.string(
           "Wall #" ++ string_of_int(index) ++ " " ++ wallType(wall##kind),
         )}
      </div>
      <button
        onClick={evt => {
          ReactEvent.Mouse.stopPropagation(evt);
          onChange([%js.deep wall["hide"].replace(!wall##hide)]);
        }}>
        {React.string(wall##hide ? "Show" : "Hide")}
      </button>
    </div>
    {selected
       ? <div
           className={Styles.join([
             Styles.column,
             Css.(style([padding2(~v=`zero, ~h=px(8))])),
           ])}>
           <div> <Triangle size=100 padding=10 /> </div>
           {Styles.spacer(8)}
           <div>
             {React.string("Absorb")}
             <Slider
               min=0
               max=1.0
               step=0.01
               value={wall##properties##absorb}
               onChange={absorb => {
                 let wall = [%js.deep
                   wall["properties"]["absorb"].replace(absorb)
                 ];
                 onChange(wall);
               }}
             />
           </div>
           {Styles.spacer(8)}
           <div>
             {React.string("Reflect vs Refract")}
             <Slider
               min=0
               max=1.0
               step=0.01
               value={wall##properties##reflect}
               onChange={reflect => {
                 let wall = [%js.deep
                   wall["properties"]["reflect"].replace(reflect)
                 ];
                 onChange(wall);
               }}
             />
           </div>
           {Styles.spacer(8)}
           <div>
             {React.string("Index of Refraction")}
             <LogSlider
               min=0
               max=5.0
               step=0.01
               value={wall##properties##refraction}
               onChange={value => {
                 let wall = [%js.deep
                   wall["properties"]["refraction"].replace(value)
                 ];
                 onChange(wall);
               }}
             />
           </div>
           {Styles.spacer(8)}
           <div>
             {React.string("Roughness")}
             <Slider
               min=0
               max=1.0
               step=0.01
               value={wall##properties##roughness}
               onChange={value => {
                 let wall = [%js.deep
                   wall["properties"]["roughness"].replace(value)
                 ];
                 onChange(wall);
               }}
             />
           </div>
           {Styles.spacer(8)}
           <button onClick={evt => onRemove()}>
             {React.string("Delete")}
           </button>
         </div>
       : React.null}
  </div>;
};