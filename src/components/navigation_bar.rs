#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::navigation_bar_style::STYLE;

const IMG_BANNER: Asset = asset!("/assets/multiplatform-light.svg");

#[component]
pub fn NavigationBar() -> Element {
    rsx! {
    style { {STYLE} }
    div { class: "container mx-auto",
                div { class: "flex justify-between items-center pt-[5%] col-lg-12 col-md-12 col-sm-12 flex-col-reverse lg:flex-row-reverse xl:flex-row grid lg:grid-rows-none lg:grid-cols-4 grid-rows-4",

                    div { class: "md:col-span-2 md:row-span-4 lg:row-span-2 row-span-4 order-first lg:order-last",
                        div { class: "lg:relative lg:w-[671px] lg:h-[671px]",
                            img { class: "inset-0 object-cover",
                                src: "{IMG_BANNER}"
                            }
                        }
                    }

                    div { class: "lg:p-[50px] text-center lg:text-start leading-none md:col-span-2 lg:row-span-2",

                        h1 { class: "title text-[48px] sm:text-[48px] lg:text-[96px] font-bold",
                            span { class: "text-[#00A8D6]", "Dioxus" }
                            span { class: "text-[#E96020] ml-2", " v0.6" }
                        }

                        h2 { class: "topic text-[#2D323B] lg:text-[96px] font-bold text-[48px] sm:text-[48px]", " Cross Platform" }

                        p { class: "des mt-10 lg:text-[24px] text-[21px] sm:text-[21px] text-balance leading-normal",
                            "Dioxus is a framework for building cross-platform apps that run on desktop, web, mobile and more."
                        }

                        // Buttons to open links in a new tab
                        div { class: "seeMore w-full flex justify-center lg:justify-start mt-5",

                            div { class: "mr-4",
                                a {
                                    href: "https://dioxuslabs.com/learn/0.6/getting_started/",
                                    target: "_blank",
                                    class: "hover:-translate-y-2 w-[150px] h-[50px] bg-[#00A8D6] text-white rounded-md flex items-center justify-center",
                                    "Read the docs"
                                }
                            }

                            div {
                                a {
                                    href: "https://discord.gg/XgGxMSkvUM",
                                    target: "_blank",
                                    class: "hover:-translate-y-2 w-[150px] h-[50px] bg-[#E96020] text-white rounded-md flex items-center justify-center",
                                    "Join Discord"
                                }
                            }

                        }
                    }

                }
            }
    }
}
