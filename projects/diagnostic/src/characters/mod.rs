/// Possible character sets to use when rendering diagnostics.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltinSymbol {
    /// Unicode characters (an attempt is made to use only commonly-supported characters).
    Unicode,
    /// ASCII-only characters.
    Ascii,
}

pub trait CharacterSet {
    fn get_characters(&self) -> Characters;
}

impl CharacterSet for BuiltinSymbol {
    fn get_characters(&self) -> Characters {
        match self {
            BuiltinSymbol::Unicode => Self {
                hbar: '─',
                vbar: '│',
                xbar: '┼',
                vbar_break: '┆',
                vbar_gap: '┆',
                uarrow: '🭯',
                rarrow: '▶',
                ltop: '╭',
                mtop: '┬',
                rtop: '╮',
                lbot: '╰',
                mbot: '┴',
                rbot: '╯',
                lbox: '[',
                rbox: ']',
                lcross: '├',
                rcross: '┤',
                underbar: '┬',
                underline: '─',
            },
            BuiltinSymbol::Ascii => Self {
                hbar: '-',
                vbar: '|',
                xbar: '+',
                vbar_break: '*',
                vbar_gap: ':',
                uarrow: '^',
                rarrow: '>',
                ltop: ',',
                mtop: 'v',
                rtop: '.',
                lbot: '`',
                mbot: '^',
                rbot: '\'',
                lbox: '[',
                rbox: ']',
                lcross: '|',
                rcross: '|',
                underbar: '|',
                underline: '^',
            },
        }
    }
}

pub struct Characters {
    pub hbar: char,
    pub vbar: char,
    pub xbar: char,
    pub vbar_break: char,
    pub vbar_gap: char,

    pub uarrow: char,
    pub rarrow: char,

    pub ltop: char,
    pub mtop: char,
    pub rtop: char,
    pub lbot: char,
    pub rbot: char,
    pub mbot: char,

    pub lbox: char,
    pub rbox: char,

    pub lcross: char,
    pub rcross: char,

    pub underbar: char,
    pub underline: char,
}

impl Characters {
    pub fn unicode() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            xbar: '┼',
            vbar_break: '┆',
            vbar_gap: '┆',
            uarrow: '🭯',
            rarrow: '▶',
            ltop: '╭',
            mtop: '┬',
            rtop: '╮',
            lbot: '╰',
            mbot: '┴',
            rbot: '╯',
            lbox: '[',
            rbox: ']',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
        }
    }

    pub fn ascii() -> Self {
        Self {
            hbar: '-',
            vbar: '|',
            xbar: '+',
            vbar_break: '*',
            vbar_gap: ':',
            uarrow: '^',
            rarrow: '>',
            ltop: ',',
            mtop: 'v',
            rtop: '.',
            lbot: '`',
            mbot: '^',
            rbot: '\'',
            lbox: '[',
            rbox: ']',
            lcross: '|',
            rcross: '|',
            underbar: '|',
            underline: '^',
        }
    }
}
