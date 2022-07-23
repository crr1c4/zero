import { defineConfig } from "vitepress";

export default defineConfig({
  title: "The Zero Programming Lenguage",
  lang: "en-us",
  themeConfig: {
    siteTitle: "The Zero Programming Lenguage",
    nav: [{ text: "Repository", link: "https://github.com/crr1c4/zero" }],

    sidebar: [
      {
        text: "Basics",
        collapsed: true,
        collapsible: true,
        items: [
          { text: "Introduction", link: "basics/" },
          { text: "Data types", link: "basics/datatypes" },
          { text: "Functions", link: "basics/functions" },
          { text: "Keywords", link: "basics/keywords" },
        ],
      },
      {
        text: "Moons 🌖",
        collapsed: true,
        collapsible: true,
        items: [
          { text: "Introduction", link: "moons/" },
          { text: "async", link: "moons/async" },
          { text: "collections", link: "moons/collections" },
          { text: "env", link: "moons/env" },
          { text: "fmt", link: "moons/fmt" },
          { text: "http", link: "moons/http" },
          { text: "io", link: "moons/io" },
          { text: "iter", link: "moons/iter" },
          { text: "math", link: "moons/math" },
          { text: "net", link: "moons/net" },
          { text: "str", link: "moons/str" },
          { text: "web", link: "moons/web" },
        ],
      },
    ],
    footer: {
      message: "Released under the MIT license",
      copyright: "Copyright © 2022 crr1c4",
    },
  },
});
