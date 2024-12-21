use maud::{html, PreEscaped, DOCTYPE};
use rand::Rng;

use crate::web::webpages::{
    blog::{blogvec, ToPreEscaped},
    icon, script, stylesheet, title,
};

use super::Webpages;

impl Webpages {
    #[expect(non_snake_case)]
    pub fn Home() -> String {
        let i = 0;

        html! {
                (DOCTYPE)
                html{
                head{
                    (stylesheet("index.css"))
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
    #[expect(non_snake_case)]
    pub fn Index() -> String {
        let pcSpecs = "Windows 10 Desktop (i9 9900k, rx 7800xt, 48 gb ram)";
        let laptopSpecs =
            "Dell Inspiron 3537 running Arch Linux with the KDE Plasma Window Manager";

        html! {
                (DOCTYPE)
                html{
                head{
                    (title("Burg's Room"))
                    (stylesheet("index.css"))
                    (icon("favicon.png"))
                }
                body{
                    (PreEscaped(r#"<meta name="description" content="burg's website">"#))
                    h1 class="heading" {("burgburg.net")};
                    div class="main"{
                        div class="outerboxes"{
                            div class="innerboxes"{
                                h1 {"Burg's Room"};
                                img src="/indexassets/wor_webszsite.png" style="width: 21%; float: left; border: 3px solid #44415a; background-color: #e0b8bb; margin-right: 6px; margin-left: 19px;";
                                p style="align-items: center; display: inline"{
                                    br;"Welcome to my room! my personal website & blog to hold my web experiences and other fun things."
                                    br;"I love to drink Coffee with Boba, and many different Hot Teas (my fav is English Breakfast)."
                                    br;"Oatmeal is the best breakfast for when i eat breakfast."
                                    br;"Forever eepy.";
                                    br;
                                    br;
                                    br;
                                }
                                h3 {"A Little About Me:"};
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
                                    "I daily drive a " {(pcSpecs)}
                                    br;
                                    "My laptop is "{(laptopSpecs)}
                                };
                                h3 {"My Favorite Shows:"};
                                p{
                                    "Girls Band Cry, " br;
                                    "Lucky Star, " br;
                                    "Bocchi The Rock," br;
                                    "Serial Expiriments Lain," br;
                                    "Better Call " a style="color: red;" onclick="SaulGoodmanBwawhAudio();"  { i {"Saul"}} "," br;
                                    "Madoka Magica"
                                }
                                //make this automated with iters maybe
                                h3 {"My Favorite Games:"};
                                p{
                                    "Vicky 3, " br;
                                    "Escape from Tarkov, " br;
                                    "VRChat," br;
                                    "SMT/Persona," br;
                                    "Hoi4" br;
                                    "Terraria, " br;
                                    "Half-Life, " br;
                                    "Arma Reforger, " br;
                                    "PSO2:NGS, " br;
                                    "Team Fortress 2"
                                }

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
                            (match blogvec().into_iter().last() {
                                Some(x) => {x.to_pre_escaped()},
                                None => {PreEscaped(String::new())},
                            })
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
                                p {a href="/blog" target="_blank" {"The Burgblog"}};
                                p {a href="/BlahajGallery" target="_blank" {"The Blahaj Gallery"}};
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
