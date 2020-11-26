# Subscript - Publishing using Web Technologies

> Note, originally Subscript referred to a frontend UI framework, but that has been abandoned, I’m recycling the old Subscript name for a new project. The old project can be found here [colbyn/subscript-old](https://github.com/colbyn/subscript-old).


## What is Subscript?

- If you are a web developer:
    - Subscript is a akin web application bundlers such as Parcel, but is -better suited- for mostly static content publishing. For those who say otherwise, see my old [GitHub repository (colbyn/school-notes)](https://github.com/colbyn/school-notes), using Parcel resulted in significant friction from a multitude of problems, notably being that Parcel and PostHTML do not interact very well, especially with nested `<include>` resources and relative file paths.
      + For example, module A and B both include module C, where module C includes asset D. PostHTML processes `<include>` files in a top-down manner, so therefore, after inlining module C in A and B -A and B now reference module asset D, using a file path relative to module C... You can imagine Parcel would then throw errors at this... Subscript by contract mostly works in a **bottom-up** manner, where module C is processed first, then modules A and B.

- If you are from academia:
    - Subscript is akin to LaTeX, both are markup languages for displaying content.

      Furthermore both are geard towards **STEM based content** by default (unlike the [Sile typesetter](https://sile-typesetter.org)<sup>†</sup> that doen't support e.g. mathematical notation).

      Yet Subscript is based on web technologies, and therefore can leverage the colossus ecosystem that makes up the web. For instance, need to display a graph of `y=x^2`? Just use a macro that embeds Desmos, and therein simply write:
      ```html
      <desmos height="200px">
          <expr>y=x^2</expr>
      </desmos>
      ```
      > Note that the above is still being migrated over from my old [GitHub repository (colbyn/school-notes)](https://github.com/colbyn/school-notes). 
      
      Or, do you need to embed musical notation? Create a macro that embeds [VexFlow](https://www.vexflow.com/). 

      Furthermore Subscript macros are more powerful than LaTeX, because Subscript macros can access the entire html tree, which is useful for e.g. the `<toc>` macro that includes a generated table of contents at the given location. 

      <sup>[†]:</sup> Regarding Sile and it's innovative layout system, since Subscript is based on web technologies, it can offer responsive grid layouts for different PDF resolutions.
      


## High Level TODO:

- [CSS Paged Media](https://www.w3.org/TR/css-page-3/): support traditional print use cases, or just rendering to a PDF. This is what I am currently planning on using for rendering to e.g. PDFs: [PagedJS](https://www.pagedjs.org)

## Example

> FYI, you can compile the following with either
> ```bash
> cargo run -- compile --root=example --input example/pages/**/*.html --output=example/output --trim pages
> ```
> or,
> ```bash
> subscript compile --root=example --input example/pages/**/*.html --output=example/output --trim pages
> ```

The following file from `./example/pages/index.html`:

```html
<include src="../template/base.html">
    <h1>Hello World</h1>
    <items>
        <p>First Paragraph</p>
        <p>Second Paragraph</p>
        <li>Third Item</li>
    </items>
</include>
```

Transforms to:

```html
<html>
  <head>
    <title>Note</title>
  </head>
  <body>
    <h1>Hello World</h1>
    <ul>
      <li>
        <p>First Paragraph</p>
      </li>
      <li>
        <p>Second Paragraph</p>
      </li>
      <li>Third Item</li>
    </ul>
  </body>
</html>
```

## Math Preview

![Preview](assets/preview.png)

Comes with a syntax highlighting extension for VS Code.

![VS-Code Preview](assets/preview-vscode-plugin.png)

It injects the LaTeX grammar from [latex-workshop](https://marketplace.visualstudio.com/items?itemName=James-Yu.latex-workshop) into the `<tex>` and `<texblock>` html tags. 

## Macros

> For autocomplete in VS-Code, for now, use the custom data definitions in [editors/vscode-html-macros](editors/vscode-html-macros/html-macros.json).

Versatility in Subscript is made possible VIA macros (the syntax is akin to web components, but it's expanded out at compile time compared to runtime, i.e. a macro).

For example, to display math formulas, you may use the `<tex>` macro, to plot `y = x^2`, you may use e.g. the `<desmos>` macro, and etc. Furthermore, say you wanted to publish content with music notation, you could create a macro that provides a high level interface to e.g. [VexFlow](https://www.vexflow.com/). 

For now, all supported macros are implemented in the core compiler.

Long term wise, I'd like to move away from the current monolithic architecture, and support extensibility in a more general manner VIA some scripting language. For this, I plan on embedding [Deno](https://github.com/denoland/deno), which will provide a multitude of benefits that e.g. NodeJS can't offer (Deno is from the creator of NodeJS).



## Cool Features (Using Macros)

### Graphing VIA Desmos/GeoGebra

> Still being copied over from the original ad-hoc implementation ([over here](https://github.com/colbyn/school-notes)). I think the API can be cleaned up a bit as well, for instance, I think grid lines should be disabled by default.

```html
<desmos height="200px">
    <expr>y=x^2</expr>
</desmos>
<geogebra height='200' type='graphing'>
    <expr>y=x^2</expr>
</geogebra>
```

![Graphing Example](assets/preview-graphing.png)

Which is customizable (see the bottom figure):

![Graphing Example](assets/preview-graphing-2.png)
