# Subscript - Typesetting & Publishing using Web Technologies

> Note, originally Subscript referred to a frontend UI framework, but that has been abandoned, I’m recycling the old Subscript name for a new project. The old project can be found here [colbyn/subscript-old](https://github.com/colbyn/subscript-old).


## High Level TODO:

- [CSS Paged Media](https://www.w3.org/TR/css-page-3/): support traditional print use cases, or just rendering to a PDF. This is what I am currently planning on using for rendering to e.g. PDFs: [PagedJS](https://www.pagedjs.org)

## Preview

![Preview](assets/preview.png)


## Cool Features

### Graphing VIA Desmos/GeoGebra

> Still being copied over from the original ad-hoc implementation ([over here](https://github.com/colbyn/school-notes)).

```html
<desmos height="200px">
    <cmd>y=x^2</cmd>
</desmos>
<geogebra height='200' type='graphing'>
    <cmd>y=x^2</cmd>
</geogebra>
```

![Graphing Example](assets/preview-graphing.png)

Which is customizable (see the bottom figure):

![Graphing Example](assets/preview-graphing-2.png)
