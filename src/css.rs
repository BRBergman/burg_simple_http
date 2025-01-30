//doesnt work with any atribute with a - in it :(
macro_rules! css {
    ($($name:ident {
        $($variant:ident: $val:ident;)*})*
    ) => {
            stringify!($( $name {$($variant: $val;)*})*)
    };
    /*($($name:ident {
        $($variant:ident-$varient2:ident: $val:ident;)*})*
    ) => {
            stringify!($( $name {$($variant-$varient2: $val;)*})*)
    };*/
}/*
pub fn test() {
    let x = css!(
        html {
            overflow:visible;
            font-family: century-gothic, sans-serif;
            font-weight: 400;
            font-style: normal;
            text-shadow: 1px 1px 2px #6c5959;/*#617161;*/
            color: #212136;
            scrollbar-width: none;
          }
          body {
            background-image: url("/bkg_red_smol_goed.png");
            background-size: 40px;
            image-rendering: pixelated;
          }
          body div {
            image-rendering: auto;
          }
          p {
            padding-left: 8%;
            margin-top: auto;
          }
          h3 {
            padding-left: 2%;
            margin-bottom: auto;
            padding-bottom: 9px;
          }
          h1 {
            padding-left: 2%;
            margin-bottom: auto;
          }

          .main {
            margin-top: 30px;
            padding-bottom: 70px;
            display: flex;
            justify-content: space-between;
            padding-left: 20%;
            padding-right: 20%;
          }
          .outerboxes {
            width: 100%;
            padding: 5px;
            padding-top: 2px ;
          }
          .innerboxes {
            box-shadow: 0px 0px 2px 2px rgba(105, 59, 59, 0.699);
            justify-content: space-between;
            border: 3px solid #44415a;
            padding: 6px;
            background-color: #ce9191/*#bbd8bb;*/
          }

          .inbetween {
            padding: 5px;
          }
          a{
            text-shadow: 1px 1px 2px #020102;/*#617161;*/
          }
          a:link {
            color: #2d94c0;
            text-decoration: none;
          }
          a:visited {
            color: #26bccf;
          }
          a:hover {
            text-decoration: underline;
          }

          .heading{
            padding: 10px;
            margin-top: 35px;
            margin-bottom: 0px;
            text-align: center;
            font-size: 200%;
            color: #212136;
            text-decoration: underline;
          }
          .month{
            margin: 0px;
          }
    );
    println!("{}", x)
}
*/