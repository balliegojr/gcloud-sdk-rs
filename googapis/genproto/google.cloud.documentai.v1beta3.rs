/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate.
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon for the detected image annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Document represents the canonical document resource in Document Understanding
/// AI.
/// It is an interchange format that provides insights into documents and allows
/// for collaboration between users and Document Understanding AI to iterate and
/// optimize for quality.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// An IANA published MIME type (also referred to as media type). For more
    /// information, see
    /// https://www.iana.org/assignments/media-types/media-types.xhtml.
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
    /// UTF-8 encoded text in reading order from the document.
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// Styles for the [Document.text][google.cloud.documentai.v1beta3.Document.text].
    #[prost(message, repeated, tag = "5")]
    pub text_styles: ::prost::alloc::vec::Vec<document::Style>,
    /// Visual page layout for the [Document][google.cloud.documentai.v1beta3.Document].
    #[prost(message, repeated, tag = "6")]
    pub pages: ::prost::alloc::vec::Vec<document::Page>,
    /// A list of entities detected on [Document.text][google.cloud.documentai.v1beta3.Document.text]. For document shards,
    /// entities in this list may cross shard boundaries.
    #[prost(message, repeated, tag = "7")]
    pub entities: ::prost::alloc::vec::Vec<document::Entity>,
    /// Relationship among [Document.entities][google.cloud.documentai.v1beta3.Document.entities].
    #[prost(message, repeated, tag = "8")]
    pub entity_relations: ::prost::alloc::vec::Vec<document::EntityRelation>,
    /// A list of translations on [Document.text][google.cloud.documentai.v1beta3.Document.text]. For document shards,
    /// translations in this list may cross shard boundaries.
    #[prost(message, repeated, tag = "12")]
    pub translations: ::prost::alloc::vec::Vec<document::Translation>,
    /// A list of text corrections made to [Document.text].  This is usually
    /// used for annotating corrections to OCR mistakes.  Text changes for a given
    /// revision may not overlap with each other.
    #[prost(message, repeated, tag = "14")]
    pub text_changes: ::prost::alloc::vec::Vec<document::TextChange>,
    /// Information about the sharding if this document is sharded part of a larger
    /// document. If the document is not sharded, this message is not specified.
    #[prost(message, optional, tag = "9")]
    pub shard_info: ::core::option::Option<document::ShardInfo>,
    /// Any error that occurred while processing this document.
    #[prost(message, optional, tag = "10")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Revision history of this document.
    #[prost(message, repeated, tag = "13")]
    pub revisions: ::prost::alloc::vec::Vec<document::Revision>,
    /// Original source document from the user.
    #[prost(oneof = "document::Source", tags = "1, 2")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// For a large document, sharding may be performed to produce several
    /// document shards. Each document shard contains this field to detail which
    /// shard it is.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardInfo {
        /// The 0-based index of this shard.
        #[prost(int64, tag = "1")]
        pub shard_index: i64,
        /// Total number of shards.
        #[prost(int64, tag = "2")]
        pub shard_count: i64,
        /// The index of the first character in [Document.text][google.cloud.documentai.v1beta3.Document.text] in the overall
        /// document global text.
        #[prost(int64, tag = "3")]
        pub text_offset: i64,
    }
    /// Annotation for common text style attributes. This adheres to CSS
    /// conventions as much as possible.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Style {
        /// Text anchor indexing into the [Document.text][google.cloud.documentai.v1beta3.Document.text].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Text color.
        #[prost(message, optional, tag = "2")]
        pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
        /// Text background color.
        #[prost(message, optional, tag = "3")]
        pub background_color: ::core::option::Option<super::super::super::super::r#type::Color>,
        /// Font weight. Possible values are normal, bold, bolder, and lighter.
        /// https://www.w3schools.com/cssref/pr_font_weight.asp
        #[prost(string, tag = "4")]
        pub font_weight: ::prost::alloc::string::String,
        /// Text style. Possible values are normal, italic, and oblique.
        /// https://www.w3schools.com/cssref/pr_font_font-style.asp
        #[prost(string, tag = "5")]
        pub text_style: ::prost::alloc::string::String,
        /// Text decoration. Follows CSS standard.
        /// <text-decoration-line> <text-decoration-color> <text-decoration-style>
        /// https://www.w3schools.com/cssref/pr_text_text-decoration.asp
        #[prost(string, tag = "6")]
        pub text_decoration: ::prost::alloc::string::String,
        /// Font size.
        #[prost(message, optional, tag = "7")]
        pub font_size: ::core::option::Option<style::FontSize>,
    }
    /// Nested message and enum types in `Style`.
    pub mod style {
        /// Font size with unit.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FontSize {
            /// Font size for the text.
            #[prost(float, tag = "1")]
            pub size: f32,
            /// Unit for the font size. Follows CSS naming (in, px, pt, etc.).
            #[prost(string, tag = "2")]
            pub unit: ::prost::alloc::string::String,
        }
    }
    /// A page in a [Document][google.cloud.documentai.v1beta3.Document].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Page {
        /// 1-based index for current [Page][google.cloud.documentai.v1beta3.Document.Page] in a parent [Document][google.cloud.documentai.v1beta3.Document].
        /// Useful when a page is taken out of a [Document][google.cloud.documentai.v1beta3.Document] for individual
        /// processing.
        #[prost(int32, tag = "1")]
        pub page_number: i32,
        /// Rendered image for this page. This image is preprocessed to remove any
        /// skew, rotation, and distortions such that the annotation bounding boxes
        /// can be upright and axis-aligned.
        #[prost(message, optional, tag = "13")]
        pub image: ::core::option::Option<page::Image>,
        /// Transformation matrices that were applied to the original document image
        /// to produce [Page.image][google.cloud.documentai.v1beta3.Document.Page.image].
        #[prost(message, repeated, tag = "14")]
        pub transforms: ::prost::alloc::vec::Vec<page::Matrix>,
        /// Physical dimension of the page.
        #[prost(message, optional, tag = "2")]
        pub dimension: ::core::option::Option<page::Dimension>,
        /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for the page.
        #[prost(message, optional, tag = "3")]
        pub layout: ::core::option::Option<page::Layout>,
        /// A list of detected languages together with confidence.
        #[prost(message, repeated, tag = "4")]
        pub detected_languages: ::prost::alloc::vec::Vec<page::DetectedLanguage>,
        /// A list of visually detected text blocks on the page.
        /// A block has a set of lines (collected into paragraphs) that have a common
        /// line-spacing and orientation.
        #[prost(message, repeated, tag = "5")]
        pub blocks: ::prost::alloc::vec::Vec<page::Block>,
        /// A list of visually detected text paragraphs on the page.
        /// A collection of lines that a human would perceive as a paragraph.
        #[prost(message, repeated, tag = "6")]
        pub paragraphs: ::prost::alloc::vec::Vec<page::Paragraph>,
        /// A list of visually detected text lines on the page.
        /// A collection of tokens that a human would perceive as a line.
        #[prost(message, repeated, tag = "7")]
        pub lines: ::prost::alloc::vec::Vec<page::Line>,
        /// A list of visually detected tokens on the page.
        #[prost(message, repeated, tag = "8")]
        pub tokens: ::prost::alloc::vec::Vec<page::Token>,
        /// A list of detected non-text visual elements e.g. checkbox,
        /// signature etc. on the page.
        #[prost(message, repeated, tag = "9")]
        pub visual_elements: ::prost::alloc::vec::Vec<page::VisualElement>,
        /// A list of visually detected tables on the page.
        #[prost(message, repeated, tag = "10")]
        pub tables: ::prost::alloc::vec::Vec<page::Table>,
        /// A list of visually detected form fields on the page.
        #[prost(message, repeated, tag = "11")]
        pub form_fields: ::prost::alloc::vec::Vec<page::FormField>,
    }
    /// Nested message and enum types in `Page`.
    pub mod page {
        /// Dimension for the page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Dimension {
            /// Page width.
            #[prost(float, tag = "1")]
            pub width: f32,
            /// Page height.
            #[prost(float, tag = "2")]
            pub height: f32,
            /// Dimension unit.
            #[prost(string, tag = "3")]
            pub unit: ::prost::alloc::string::String,
        }
        /// Rendered image contents for this page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Image {
            /// Raw byte content of the image.
            #[prost(bytes = "vec", tag = "1")]
            pub content: ::prost::alloc::vec::Vec<u8>,
            /// Encoding mime type for the image.
            #[prost(string, tag = "2")]
            pub mime_type: ::prost::alloc::string::String,
            /// Width of the image in pixels.
            #[prost(int32, tag = "3")]
            pub width: i32,
            /// Height of the image in pixels.
            #[prost(int32, tag = "4")]
            pub height: i32,
        }
        /// Representation for transformation matrix, intended to be compatible and
        /// used with OpenCV format for image manipulation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Matrix {
            /// Number of rows in the matrix.
            #[prost(int32, tag = "1")]
            pub rows: i32,
            /// Number of columns in the matrix.
            #[prost(int32, tag = "2")]
            pub cols: i32,
            /// This encodes information about what data type the matrix uses.
            /// For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list
            /// of OpenCV primitive data types, please refer to
            /// https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html
            #[prost(int32, tag = "3")]
            pub r#type: i32,
            /// The matrix data.
            #[prost(bytes = "vec", tag = "4")]
            pub data: ::prost::alloc::vec::Vec<u8>,
        }
        /// Visual element describing a layout unit on a page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Layout {
            /// Text anchor indexing into the [Document.text][google.cloud.documentai.v1beta3.Document.text].
            #[prost(message, optional, tag = "1")]
            pub text_anchor: ::core::option::Option<super::TextAnchor>,
            /// Confidence of the current [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] within context of the object this
            /// layout is for. e.g. confidence can be for a single token, a table,
            /// a visual element, etc. depending on context. Range [0, 1].
            #[prost(float, tag = "2")]
            pub confidence: f32,
            /// The bounding polygon for the [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout].
            #[prost(message, optional, tag = "3")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
            /// Detected orientation for the [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout].
            #[prost(enumeration = "layout::Orientation", tag = "4")]
            pub orientation: i32,
        }
        /// Nested message and enum types in `Layout`.
        pub mod layout {
            /// Detected human reading orientation.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Orientation {
                /// Unspecified orientation.
                Unspecified = 0,
                /// Orientation is aligned with page up.
                PageUp = 1,
                /// Orientation is aligned with page right.
                /// Turn the head 90 degrees clockwise from upright to read.
                PageRight = 2,
                /// Orientation is aligned with page down.
                /// Turn the head 180 degrees from upright to read.
                PageDown = 3,
                /// Orientation is aligned with page left.
                /// Turn the head 90 degrees counterclockwise from upright to read.
                PageLeft = 4,
            }
        }
        /// A block has a set of lines (collected into paragraphs) that have a
        /// common line-spacing and orientation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Block {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [Block][google.cloud.documentai.v1beta3.Document.Page.Block].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The history of this annotation.
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A collection of lines that a human would perceive as a paragraph.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Paragraph {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [Paragraph][google.cloud.documentai.v1beta3.Document.Page.Paragraph].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A collection of tokens that a human would perceive as a line.
        /// Does not cross column boundaries, can be horizontal, vertical, etc.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Line {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [Line][google.cloud.documentai.v1beta3.Document.Page.Line].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A detected token.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Token {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [Token][google.cloud.documentai.v1beta3.Document.Page.Token].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Detected break at the end of a [Token][google.cloud.documentai.v1beta3.Document.Page.Token].
            #[prost(message, optional, tag = "2")]
            pub detected_break: ::core::option::Option<token::DetectedBreak>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[prost(message, optional, tag = "4")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// Nested message and enum types in `Token`.
        pub mod token {
            /// Detected break at the end of a [Token][google.cloud.documentai.v1beta3.Document.Page.Token].
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DetectedBreak {
                /// Detected break type.
                #[prost(enumeration = "detected_break::Type", tag = "1")]
                pub r#type: i32,
            }
            /// Nested message and enum types in `DetectedBreak`.
            pub mod detected_break {
                /// Enum to denote the type of break found.
                #[derive(
                    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
                )]
                #[repr(i32)]
                pub enum Type {
                    /// Unspecified break type.
                    Unspecified = 0,
                    /// A single whitespace.
                    Space = 1,
                    /// A wider whitespace.
                    WideSpace = 2,
                    /// A hyphen that indicates that a token has been split across lines.
                    Hyphen = 3,
                }
            }
        }
        /// Detected non-text visual elements e.g. checkbox, signature etc. on the
        /// page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VisualElement {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [VisualElement][google.cloud.documentai.v1beta3.Document.Page.VisualElement].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Type of the [VisualElement][google.cloud.documentai.v1beta3.Document.Page.VisualElement].
            #[prost(string, tag = "2")]
            pub r#type: ::prost::alloc::string::String,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A table representation similar to HTML table structure.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Table {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [Table][google.cloud.documentai.v1beta3.Document.Page.Table].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Header rows of the table.
            #[prost(message, repeated, tag = "2")]
            pub header_rows: ::prost::alloc::vec::Vec<table::TableRow>,
            /// Body rows of the table.
            #[prost(message, repeated, tag = "3")]
            pub body_rows: ::prost::alloc::vec::Vec<table::TableRow>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "4")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// Nested message and enum types in `Table`.
        pub mod table {
            /// A row of table cells.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableRow {
                /// Cells that make up this row.
                #[prost(message, repeated, tag = "1")]
                pub cells: ::prost::alloc::vec::Vec<TableCell>,
            }
            /// A cell representation inside the table.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableCell {
                /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for [TableCell][google.cloud.documentai.v1beta3.Document.Page.Table.TableCell].
                #[prost(message, optional, tag = "1")]
                pub layout: ::core::option::Option<super::Layout>,
                /// How many rows this cell spans.
                #[prost(int32, tag = "2")]
                pub row_span: i32,
                /// How many columns this cell spans.
                #[prost(int32, tag = "3")]
                pub col_span: i32,
                /// A list of detected languages together with confidence.
                #[prost(message, repeated, tag = "4")]
                pub detected_languages: ::prost::alloc::vec::Vec<super::DetectedLanguage>,
            }
        }
        /// A form field detected on the page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FormField {
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for the [FormField][google.cloud.documentai.v1beta3.Document.Page.FormField] name. e.g. `Address`, `Email`,
            /// `Grand total`, `Phone number`, etc.
            #[prost(message, optional, tag = "1")]
            pub field_name: ::core::option::Option<Layout>,
            /// [Layout][google.cloud.documentai.v1beta3.Document.Page.Layout] for the [FormField][google.cloud.documentai.v1beta3.Document.Page.FormField] value.
            #[prost(message, optional, tag = "2")]
            pub field_value: ::core::option::Option<Layout>,
            /// A list of detected languages for name together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub name_detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// A list of detected languages for value together with confidence.
            #[prost(message, repeated, tag = "4")]
            pub value_detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// If the value is non-textual, this field represents the type. Current
            /// valid values are:
            /// - blank (this indicates the field_value is normal text)
            /// - "unfilled_checkbox"
            /// - "filled_checkbox"
            #[prost(string, tag = "5")]
            pub value_type: ::prost::alloc::string::String,
        }
        /// Detected language for a structural component.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DetectedLanguage {
            /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
            /// information, see
            /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
            #[prost(string, tag = "1")]
            pub language_code: ::prost::alloc::string::String,
            /// Confidence of detected language. Range [0, 1].
            #[prost(float, tag = "2")]
            pub confidence: f32,
        }
    }
    /// A phrase in the text that is a known entity type, such as a person, an
    /// organization, or location.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Provenance of the entity.
        /// Text anchor indexing into the [Document.text][google.cloud.documentai.v1beta3.Document.text].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Entity type from a schema e.g. `Address`.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Text value in the document e.g. `1600 Amphitheatre Pkwy`.
        #[prost(string, tag = "3")]
        pub mention_text: ::prost::alloc::string::String,
        /// Deprecated.  Use `id` field instead.
        #[prost(string, tag = "4")]
        pub mention_id: ::prost::alloc::string::String,
        /// Optional. Confidence of detected Schema entity. Range [0, 1].
        #[prost(float, tag = "5")]
        pub confidence: f32,
        /// Optional. Represents the provenance of this entity wrt. the location on the
        /// page where it was found.
        #[prost(message, optional, tag = "6")]
        pub page_anchor: ::core::option::Option<PageAnchor>,
        /// Canonical id. This will be a unique value in the entity list
        /// for this document.
        #[prost(string, tag = "7")]
        pub id: ::prost::alloc::string::String,
        /// Optional. Normalized entity value. Absent if the extracted value could not be
        /// converted or the type (e.g. address) is not supported for certain
        /// parsers. This field is also only populated for certain supported document
        /// types.
        #[prost(message, optional, tag = "9")]
        pub normalized_value: ::core::option::Option<entity::NormalizedValue>,
        /// Optional. Entities can be nested to form a hierarchical data structure representing
        /// the content in the document.
        #[prost(message, repeated, tag = "10")]
        pub properties: ::prost::alloc::vec::Vec<Entity>,
        /// Optional. The history of this annotation.
        #[prost(message, optional, tag = "11")]
        pub provenance: ::core::option::Option<Provenance>,
        /// Optional. Whether the entity will be redacted for de-identification purposes.
        #[prost(bool, tag = "12")]
        pub redacted: bool,
    }
    /// Nested message and enum types in `Entity`.
    pub mod entity {
        /// Parsed and normalized entity value.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NormalizedValue {
            /// Required. Normalized entity value stored as a string. This field is populated for
            /// supported document type (e.g. Invoice). For some entity types, one of
            /// respective 'structured_value' fields may also be populated.
            ///
            /// - Money/Currency type (`money_value`) is in the ISO 4217 text format.
            /// - Date type (`date_value`) is in the ISO 8601 text format.
            /// - Datetime type (`datetime_value`) is in the ISO 8601 text format.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
            /// Structured entity value. Must match entity type defined in schema if
            /// known. If this field is present, the 'text' field is still populated.
            #[prost(oneof = "normalized_value::StructuredValue", tags = "2, 3, 4, 5")]
            pub structured_value: ::core::option::Option<normalized_value::StructuredValue>,
        }
        /// Nested message and enum types in `NormalizedValue`.
        pub mod normalized_value {
            /// Structured entity value. Must match entity type defined in schema if
            /// known. If this field is present, the 'text' field is still populated.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum StructuredValue {
                /// Money value. See also:
                ///
                /// https:
                /// github.com/googleapis/googleapis/blob/master/google/type/money.proto
                #[prost(message, tag = "2")]
                MoneyValue(super::super::super::super::super::super::r#type::Money),
                /// Date value. Includes year, month, day. See also:
                ///
                /// https:
                /// github.com/googleapis/googleapis/blob/master/google/type/date.proto
                #[prost(message, tag = "3")]
                DateValue(super::super::super::super::super::super::r#type::Date),
                /// DateTime value. Includes date, time, and timezone. See also:
                ///
                /// https:
                /// github.com/googleapis/googleapis/blob/master/google/type/datetime.proto
                #[prost(message, tag = "4")]
                DatetimeValue(super::super::super::super::super::super::r#type::DateTime),
                /// Postal address. See also:
                ///
                /// https:
                /// github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto
                #[prost(message, tag = "5")]
                AddressValue(super::super::super::super::super::super::r#type::PostalAddress),
            }
        }
    }
    /// Relationship between [Entities][google.cloud.documentai.v1beta3.Document.Entity].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityRelation {
        /// Subject entity id.
        #[prost(string, tag = "1")]
        pub subject_id: ::prost::alloc::string::String,
        /// Object entity id.
        #[prost(string, tag = "2")]
        pub object_id: ::prost::alloc::string::String,
        /// Relationship description.
        #[prost(string, tag = "3")]
        pub relation: ::prost::alloc::string::String,
    }
    /// A translation of the text segment.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Translation {
        /// Provenance of the translation.
        /// Text anchor indexing into the [Document.text][google.cloud.documentai.v1beta3.Document.text].  There can only be a
        /// single `TextAnchor.text_segments` element.  If the start and
        /// end index of the text segment are the same, the text change is inserted
        /// before that index.
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
        /// information, see
        /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
        #[prost(string, tag = "2")]
        pub language_code: ::prost::alloc::string::String,
        /// Text translated into the target language.
        #[prost(string, tag = "3")]
        pub translated_text: ::prost::alloc::string::String,
        /// The history of this annotation.
        #[prost(message, repeated, tag = "4")]
        pub provenance: ::prost::alloc::vec::Vec<Provenance>,
    }
    /// Text reference indexing into the [Document.text][google.cloud.documentai.v1beta3.Document.text].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAnchor {
        /// The text segments from the [Document.text][google.cloud.documentai.v1beta3.Document.text].
        #[prost(message, repeated, tag = "1")]
        pub text_segments: ::prost::alloc::vec::Vec<text_anchor::TextSegment>,
        /// Contains the content of the text span so that users do
        /// not have to look it up in the text_segments.
        #[prost(string, tag = "2")]
        pub content: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TextAnchor`.
    pub mod text_anchor {
        /// A text segment in the [Document.text][google.cloud.documentai.v1beta3.Document.text]. The indices may be out of bounds
        /// which indicate that the text extends into another document shard for
        /// large sharded documents. See [ShardInfo.text_offset][google.cloud.documentai.v1beta3.Document.ShardInfo.text_offset]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextSegment {
            /// [TextSegment][google.cloud.documentai.v1beta3.Document.TextAnchor.TextSegment] start UTF-8 char index in the [Document.text][google.cloud.documentai.v1beta3.Document.text].
            #[prost(int64, tag = "1")]
            pub start_index: i64,
            /// [TextSegment][google.cloud.documentai.v1beta3.Document.TextAnchor.TextSegment] half open end UTF-8 char index in the
            /// [Document.text][google.cloud.documentai.v1beta3.Document.text].
            #[prost(int64, tag = "2")]
            pub end_index: i64,
        }
    }
    /// Referencing the visual context of the entity in the [Document.pages][google.cloud.documentai.v1beta3.Document.pages].
    /// Page anchors can be cross-page, consist of multiple bounding polygons and
    /// optionally reference specific layout element types.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PageAnchor {
        /// One or more references to visual page elements
        #[prost(message, repeated, tag = "1")]
        pub page_refs: ::prost::alloc::vec::Vec<page_anchor::PageRef>,
    }
    /// Nested message and enum types in `PageAnchor`.
    pub mod page_anchor {
        /// Represents a weak reference to a page element within a document.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PageRef {
            /// Required. Index into the [Document.pages][google.cloud.documentai.v1beta3.Document.pages] element
            #[prost(int64, tag = "1")]
            pub page: i64,
            /// Optional. The type of the layout element that is being referenced if any.
            #[prost(enumeration = "page_ref::LayoutType", tag = "2")]
            pub layout_type: i32,
            /// Optional. Deprecated.  Use [PageRef.bounding_poly][google.cloud.documentai.v1beta3.Document.PageAnchor.PageRef.bounding_poly] instead.
            #[deprecated]
            #[prost(string, tag = "3")]
            pub layout_id: ::prost::alloc::string::String,
            /// Optional. Identifies the bounding polygon of a layout element on the page.
            #[prost(message, optional, tag = "4")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
        }
        /// Nested message and enum types in `PageRef`.
        pub mod page_ref {
            /// The type of layout that is being referenced.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum LayoutType {
                /// Layout Unspecified.
                Unspecified = 0,
                /// References a [Page.blocks][google.cloud.documentai.v1beta3.Document.Page.blocks] element.
                Block = 1,
                /// References a [Page.paragraphs][google.cloud.documentai.v1beta3.Document.Page.paragraphs] element.
                Paragraph = 2,
                /// References a [Page.lines][google.cloud.documentai.v1beta3.Document.Page.lines] element.
                Line = 3,
                /// References a [Page.tokens][google.cloud.documentai.v1beta3.Document.Page.tokens] element.
                Token = 4,
                /// References a [Page.visual_elements][google.cloud.documentai.v1beta3.Document.Page.visual_elements] element.
                VisualElement = 5,
                /// Refrrences a [Page.tables][google.cloud.documentai.v1beta3.Document.Page.tables] element.
                Table = 6,
                /// References a [Page.form_fields][google.cloud.documentai.v1beta3.Document.Page.form_fields] element.
                FormField = 7,
            }
        }
    }
    /// Structure to identify provenance relationships between annotations in
    /// different revisions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Provenance {
        /// The index of the revision that produced this element.
        #[prost(int32, tag = "1")]
        pub revision: i32,
        /// The Id of this operation.  Needs to be unique within the scope of the
        /// revision.
        #[prost(int32, tag = "2")]
        pub id: i32,
        /// References to the original elements that are replaced.
        #[prost(message, repeated, tag = "3")]
        pub parents: ::prost::alloc::vec::Vec<provenance::Parent>,
        /// The type of provenance operation.
        #[prost(enumeration = "provenance::OperationType", tag = "4")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `Provenance`.
    pub mod provenance {
        /// Structure for referencing parent provenances.  When an element replaces
        /// one of more other elements parent references identify the elements that
        /// are replaced.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Parent {
            /// The index of the [Document.revisions] identifying the parent revision.
            #[prost(int32, tag = "1")]
            pub revision: i32,
            /// The id of the parent provenance.
            #[prost(int32, tag = "2")]
            pub id: i32,
        }
        /// If a processor or agent does an explicit operation on existing elements.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum OperationType {
            /// Operation type unspecified.
            Unspecified = 0,
            /// Add an element.  Implicit if no `parents` are set for the provenance.
            Add = 1,
            /// The element is removed.  No `parents` should be set.
            Remove = 2,
            /// Explicitly replaces the element(s) identified by `parents`.
            Replace = 3,
            /// Element is requested for human review.
            EvalRequested = 4,
            /// Element is review and approved at human review, confidence will be set
            /// to 1.0
            EvalApproved = 5,
        }
    }
    /// Contains past or forward revisions of this document.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Revision {
        /// Id of the revision.  Unique within the context of the document.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The revisions that this revision is based on.  This can include one or
        /// more parent (when documents are merged.)  This field represents the
        /// index into the `revisions` field.
        #[prost(int32, repeated, tag = "2")]
        pub parent: ::prost::alloc::vec::Vec<i32>,
        /// The time that the revision was created.
        #[prost(message, optional, tag = "3")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Human Review information of this revision.
        #[prost(message, optional, tag = "6")]
        pub human_review: ::core::option::Option<revision::HumanReview>,
        /// Who/what made the change
        #[prost(oneof = "revision::Source", tags = "4, 5")]
        pub source: ::core::option::Option<revision::Source>,
    }
    /// Nested message and enum types in `Revision`.
    pub mod revision {
        /// Human Review information of the document.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HumanReview {
            /// Human review state. e.g. `requested`, `succeeded`, `rejected`.
            #[prost(string, tag = "1")]
            pub state: ::prost::alloc::string::String,
            /// A message providing more details about the current state of processing.
            /// For example, the rejection reason when the state is `rejected`.
            #[prost(string, tag = "2")]
            pub state_message: ::prost::alloc::string::String,
        }
        /// Who/what made the change
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// If the change was made by a person specify the name or id of that
            /// person.
            #[prost(string, tag = "4")]
            Agent(::prost::alloc::string::String),
            /// If the annotation was made by processor identify the processor by its
            /// resource name.
            #[prost(string, tag = "5")]
            Processor(::prost::alloc::string::String),
        }
    }
    /// This message is used for text changes aka. OCR corrections.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextChange {
        /// Provenance of the correction.
        /// Text anchor indexing into the [Document.text][google.cloud.documentai.v1beta3.Document.text].  There can only be a
        /// single `TextAnchor.text_segments` element.  If the start and
        /// end index of the text segment are the same, the text change is inserted
        /// before that index.
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// The text that replaces the text identified in the `text_anchor`.
        #[prost(string, tag = "2")]
        pub changed_text: ::prost::alloc::string::String,
        /// The history of this annotation.
        #[prost(message, repeated, tag = "3")]
        pub provenance: ::prost::alloc::vec::Vec<Provenance>,
    }
    /// Original source document from the user.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Currently supports Google Cloud Storage URI of the form
        ///    `gs://bucket_name/object_name`. Object versioning is not supported.
        ///    See [Google Cloud Storage Request
        ///    URIs](https://cloud.google.com/storage/docs/reference-uris) for more
        ///    info.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// Inline document content, represented as a stream of bytes.
        /// Note: As with all `bytes` fields, protobuffers use a pure binary
        /// representation, whereas JSON representations use base64.
        #[prost(bytes, tag = "2")]
        Content(::prost::alloc::vec::Vec<u8>),
    }
}
/// Request message for the process document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessRequest {
    /// Required. The processor resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The document payload, the [content] and [mime_type] fields must be set.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
    /// Whether Human Review feature should be skipped for this request. Default to
    /// false.
    #[prost(bool, tag = "3")]
    pub skip_human_review: bool,
}
/// Response message for the process document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessResponse {
    /// The document payload, will populate fields based on the processor's
    /// behavior.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The name of the operation triggered by the processed document. If the human
    /// review process is not triggered, this field will be empty. It has the same
    /// response type and metadata as the long running operation returned by
    /// ReviewDocument method.
    #[prost(string, tag = "2")]
    pub human_review_operation: ::prost::alloc::string::String,
}
/// Request message for batch process document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessRequest {
    /// Required. The processor resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The input config for each single document in the batch process.
    #[prost(message, repeated, tag = "2")]
    pub input_configs: ::prost::alloc::vec::Vec<batch_process_request::BatchInputConfig>,
    /// The overall output config for batch process.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::core::option::Option<batch_process_request::BatchOutputConfig>,
}
/// Nested message and enum types in `BatchProcessRequest`.
pub mod batch_process_request {
    /// The message for input config in batch process.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchInputConfig {
        /// The Cloud Storage location as the source of the document.
        #[prost(string, tag = "1")]
        pub gcs_source: ::prost::alloc::string::String,
        /// Mimetype of the input. If the input is a raw document, the supported
        /// mimetypes are application/pdf, image/tiff, and image/gif.
        /// If the input is a [Document] proto, the type should be application/json.
        #[prost(string, tag = "2")]
        pub mime_type: ::prost::alloc::string::String,
    }
    /// The message for output config in batch process.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchOutputConfig {
        /// The output Cloud Storage directory to put the processed documents.
        #[prost(string, tag = "1")]
        pub gcs_destination: ::prost::alloc::string::String,
    }
}
/// Response message for batch process document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessResponse {}
/// The long running operation metadata for batch process method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessMetadata {
    /// The state of the current batch processing.
    #[prost(enumeration = "batch_process_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of processing.
    /// For example, the error message if the operation is failed.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The list of response details of each document.
    #[prost(message, repeated, tag = "5")]
    pub individual_process_statuses:
        ::prost::alloc::vec::Vec<batch_process_metadata::IndividualProcessStatus>,
}
/// Nested message and enum types in `BatchProcessMetadata`.
pub mod batch_process_metadata {
    /// The status of a each individual document in the batch process.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndividualProcessStatus {
        /// The source of the document, same as the [input_gcs_source] field in the
        /// request when the batch process started. The batch process is started by
        /// take snapshot of that document, since a user can move or change that
        /// document during the process.
        #[prost(string, tag = "1")]
        pub input_gcs_source: ::prost::alloc::string::String,
        /// The status of the processing of the document.
        #[prost(message, optional, tag = "2")]
        pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
        /// The output_gcs_destination (in the request as 'output_gcs_destination')
        /// of the processed document if it was successful, otherwise empty.
        #[prost(string, tag = "3")]
        pub output_gcs_destination: ::prost::alloc::string::String,
        /// The name of the operation triggered by the processed document. If the
        /// human review process is not triggered, this field will be empty. It has
        /// the same response type and metadata as the long running operation
        /// returned by ReviewDocument method.
        #[prost(string, tag = "4")]
        pub human_review_operation: ::prost::alloc::string::String,
    }
    /// Possible states of the batch processing operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Request operation is waiting for scheduling.
        Waiting = 1,
        /// Request is being processed.
        Running = 2,
        /// The batch processing completed successfully.
        Succeeded = 3,
        /// The batch processing was being cancelled.
        Cancelling = 4,
        /// The batch processing was cancelled.
        Cancelled = 5,
        /// The batch processing has failed.
        Failed = 6,
    }
}
/// Request message for review document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewDocumentRequest {
    /// Required. The resource name of the HumanReviewConfig that the document will be
    /// reviewed with.
    #[prost(string, tag = "1")]
    pub human_review_config: ::prost::alloc::string::String,
    /// The document that needs human review.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
}
/// Response message for review document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewDocumentResponse {
    /// The Cloud Storage uri for the human reviewed document.
    #[prost(string, tag = "1")]
    pub gcs_destination: ::prost::alloc::string::String,
}
/// The long running operation metadata for review document method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewDocumentOperationMetadata {
    /// Used only when Operation.done is false.
    #[prost(enumeration = "review_document_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of processing.
    /// For example, the error message if the operation is failed.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ReviewDocumentOperationMetadata`.
pub mod review_document_operation_metadata {
    /// State of the longrunning operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// Operation is still running.
        Running = 1,
        /// Operation is being cancelled.
        Cancelling = 2,
        /// Operation succeeded.
        Succeeded = 3,
        /// Operation failed.
        Failed = 4,
        /// Operation is cancelled.
        Cancelled = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod document_processor_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to call Cloud DocumentAI to process documents according to the"]
    #[doc = " processor's definition. Processors are built using state-of-the-art Google"]
    #[doc = " AI such as natural language, computer vision, and translation to extract"]
    #[doc = " structured information from unstructured or semi-structured documents."]
    pub struct DocumentProcessorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentProcessorServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Processes a single document."]
        pub async fn process_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ProcessRequest>,
        ) -> Result<tonic::Response<super::ProcessResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1beta3.DocumentProcessorService/ProcessDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " LRO endpoint to batch process many documents. The output is written"]
        #[doc = " to Cloud Storage as JSON in the [Document] format."]
        pub async fn batch_process_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchProcessRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1beta3.DocumentProcessorService/BatchProcessDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Send a document for Human Review. The input document should be processed by"]
        #[doc = " the specified processor."]
        pub async fn review_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ReviewDocumentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1beta3.DocumentProcessorService/ReviewDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DocumentProcessorServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DocumentProcessorServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DocumentProcessorServiceClient {{ ... }}")
        }
    }
}
