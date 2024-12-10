package lib

type MediaType int8
type MediaSource int8

type MediaInput struct {
    Type MediaType
    Source MediaSource
}


const SupportedInput (
    MUSIC SupportedMediaType = iota
    AUDIOBOOK
    EBOOK
    MOVIE
    TVSHOW
)

