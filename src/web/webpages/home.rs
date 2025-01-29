use maud::{html, PreEscaped, DOCTYPE};
use rand::Rng;

use crate::web::webpages::{
    blog::{blogvec, ToMarkup},
    icon, script,
    style::Styles,
    title,
};

use super::Webpages;

impl Webpages {
    pub fn home(_input: Option<String>) -> String {
        let i = 0;
        html! {
                (DOCTYPE)
                html{
                head{
                    (Styles::default())
                    title{"home"}
                }
                body{
                    h1{"Home Page!"}
                    div{button href=("/")  { {(i)}}}
                    div{p1{"gugfjksfgljkgfdlkjdh"}}
                    div{"hi"}
                }
            }
        }
        .into_string()
    }
}

impl Webpages {
    fn quote_of_the_refresh() -> String {
        let list = vec![
            "bwaaa",
            "DO NOT THROW SOULS",
            "You're out of touch",
            "haiii",
            "Let me super break you",
            "Stellemarch is t4t",
            "amogus",
            "HATSUNE FUCKIN MIKUUUU",
            "It's Ikuyover",
            "It's Ryover",
            "It's murder, the solution is murder",
        ];
        let x = rand::thread_rng().gen_range(0..list.len());
        list[x].into()
    }
    //eventually rewrite with maud
    pub fn index(_input: Option<String>) -> String {
        let pc_specs = "Windows 10 Desktop (i9 9900k, rx 7800xt, 48 gb ram)";
        let laptop_specs =
            "Dell Inspiron 3537 running Arch Linux with the KDE Plasma Window Manager";

        html! {
                (DOCTYPE)
                html{
                head{
                    (title("Burg's Room"))
                    (Styles::default())
                    (icon("favicon.png"))
                }
                body{
                    (PreEscaped(r#"<meta name="description" content="burg's website">"#))
                    h1 class="heading" {("burgburg.net")};
                    div class="main"{
                        div class="outerboxes"{
                            div class="innerboxes"{
                                h1 {"Burg's Room"};
                                img src="/indexassets/teto_1.png" style="width: 21%; float: left; border: 3px solid #44415a; background-color: #e0b8bb; margin-right: 6px; margin-left: 19px;";
                                p style="align-items: center; display: inline"{
                                    br;"Welcome to my room! my personal website & blog to hold my web experiences and other fun things."
                                    br;"I love to drink Coffee with Boba, and many different Hot Teas (my fav is English Breakfast)."
                                    br;"Oatmeal is the best breakfast for when i eat breakfast."
                                    br;"Forever eepy.";
                                }
                                h3 style="clear: left"{"A Little About Me:"};
                                p {
                                    "Im a Dutch-American from the Northeast 🇺🇸."
                                    br; "Im learning Dutch 🇳🇱."
                                    br;
                                    "My Favorite colors are "
                                    span style="color: #2d5a29" {"Green"}; {", "}
                                    span style="color: #f5acba; text-shadow: 1px 1px 2px black;" {"Pink"}{", and "}
                                    span style="color: #C08BF8; text-shadow: 1px 1px 2px black;"{"Purple"}; br;
                                    {"I do some "} I {"3D"}; {" art in "} a href="https://www.blender.org/"{"Blender"}{" and "} I {"2D"};" art in " a
                                    href="https://krita.org/en/" {"Krita"}
                                    br;
                                    "My Main Coding Laguage is Rust"
                                    br;
                                    "I daily drive a " {(pc_specs)}
                                    br;
                                    "My laptop is "{(laptop_specs)}
                                };
                                h3 {"My Favorite Shows:"};
                                {(vec!["Girls Band Cry",
                                "Lucky Star",
                                "Bocchi The Rock",
                                "Serial Expiriments Lain",
                                "Better Call <a style=\"color: red;\" onclick=\"SaulGoodmanBwawhAudio();\"><i>Saul</i></a>",
                                "Madoka Magica"].as_list().unwrap())}
                                h3 {"My Favorite Games:"};
                                {(vec!["Vicky 3",
                                "Escape from Tarkov",
                                "VRChat",
                                "SMT/Persona",
                                "Hoi4",
                                "Terraria",
                                "Half-Life",
                                "Arma Reforger",
                                "PSO2:NGS",
                                "Team Fortress 2"].as_list().unwrap())}
                            }
                            div class="inbetween"{
                                marquee behavior="scroll" direction="left"{
                                    span {"ALL are welcome. Enjoy Your Stay!"}
                                }
                            }
                            div class="innerboxes"{
                                h3 {"Projects:"};
                                p{
                                    "rewriting the website in rust"
                                }
                            }
                            div class="inbetween" {br;}
                            @if let Some(x) =  blogvec().into_iter().last() {
                                (x.as_inner_boxes())
                            }
                            div class="inbetween" {br;}
                            div class="innerboxes"{
                                h3{"QOTR (quote of the refresh):"}
                                p{(Self::quote_of_the_refresh())}
                            }
                        }
                        div class="outerboxes" style="width: 25%"{//second half
                            div class="innerboxes"{
                                h3{"Links:"}
                                p {a href="https://twitter.com/_burgburg_" target="_blank" {"Twitter"}};
                                p {a href="https://bsky.app/profile/burgburg.net" target="_blank" {"Bluesky"}};
                                p {a href="https://steamcommunity.com/id/_burgburg_/" target="_blank" {"Steam"}};
                                p {a href="https://github.com/BRBergman" target="_blank" {"Github"}};
                                p {a href="https://throne.com/burgburg" target="_blank" {"Throne"}};
                            }
                            div class="inbetween" {br;}
                            div class="innerboxes"{
                                h3{"Frens:"}
                                p {a href="https://natgeo.nekoweb.org/" target="_blank" {"Natgeo"}};
                                p {a href="https://swiftersweeper.nekoweb.org/" target="_blank" {"Swift"}};
                            }
                            div class="inbetween" {br;}
                            div class="innerboxes"{
                                h3{"Pages:"}
                                p {a href="/blog" {"The Burgblog"}};
                                p {a href="/BlahajGallery" {"The Blahaj Gallery"}};
                            }
                            div class="inbetween" {br;}
                            div class="innerboxes"{
                                div style="text-align: center; image-rendering: pixelated;"{
                                    img src="https://mypillowfort.nekoweb.org/media5/tumblr_ace137e4afe10e9090c839f452444524_b175a197_75.gif" ;
                                    img src="https://mypillowfort.nekoweb.org/media5/tumblr_75f0b86c777ae5383ec55e4aaab64603_f6b25393_75.gif";
                                    img src="https://mypillowfort.nekoweb.org/media5/tumblr_9d8e9f7bea9bc75fa560df7e001c9f41_1f93276b_75.gif";
                                }
                            }
                            div class="inbetween" {br;}
                            div class="innerboxes"{
                                img src="/indexassets/Lain.jpg" style="object-fit: contain; width: 100%; height: 100%;";
                                marquee behavior="scroll" direction="left" {"Lets All Love Lain."};
                            }
                            div class="inbetween" {br;}
                            div class="innerboxes"{
                                p1 style="color: #bda4e5; text-shadow: 1px 1px 2px #44415a;"{"awawawawawaw"}
                                img src="/indexassets/luckyScream.jpg" style="text-align: center; width: 100%; image-rendering: smooth;";
                                div style="text-align: right" {p1 style="color: #748fdc; text-shadow: 1px 1px 2px #52505f; "{"ehe"}}
                            }
                        }
                    }
                }
                {(script("/index.js"))}
                {(PreEscaped(r#"<h1 id=nl>
                    <script src="https://webneko.net/n20171213.js"></script><a href="https://webneko.net">Neko</a>
                </h1>"#))}
            }
        }.into_string()
    }
}
trait AsList {
    fn as_list(self) -> Option<PreEscaped<String>>;
}
impl<T: ToString> AsList for Vec<T> {
    fn as_list(self) -> Option<PreEscaped<String>> {
        let mut x = self
            .iter()
            .map(|x| format!("{}, </br>", x.to_string()))
            .collect::<Vec<String>>();
        *x.first_mut()? = format!("<p> {}, <br>",x.first()?);
        *x.last_mut()? = format!("{}</p>", x.last()?);
        Some(PreEscaped(x.concat()))
    }
}
