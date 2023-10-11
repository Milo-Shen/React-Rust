struct ReactElement {
    // This tag allows us to uniquely identify this as a React Element
    // todo: the origin name is $$typeof
    __typeof: String,

    // Built-in properties that belong on the element
    // todo: the origin name is type
    _type: String,

    // todo: key may has different types, such as String, i32, f32 and etc...
    key: String,

    // todo: the origin name of _ref is ref
    _ref:String,

    // todo: will identify the detailed type later
    props: String,

    // Record the component responsible for creating this element.
    // todo: will identify the detailed type later
    _owner: String,
}
