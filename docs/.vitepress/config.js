import { defineConfig } from "vitepress";

export default defineConfig({
  title: "The Zero Programming Lenguage",
  lang: "en-us",
  themeConfig: {
    siteTitle: "The Zero Programming Lenguage",
    sidebar: [
      {
        text: "Moons",
        collapsed: true,
        collapsible: true,
        items: [
          { text: "Introduction", link: "/moons/" },
          { text: "async", link: "/moons/async" },
          { text: "env", link: "/moons/env" },
          { text: "fmt", link: "/moons/fmt" },
          { text: "http", link: "/moons/http" },
          { text: "io", link: "/moons/io" },
          { text: "math", link: "/moons/math" },
          { text: "net", link: "/moons/net" },
          { text: "web", link: "/moons/web" },
        ],
      },
    ],
  },
});
