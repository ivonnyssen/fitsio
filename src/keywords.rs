#[derive(Debug, PartialEq)]
pub enum Keyword {
    Simple,
    BitPix,
    Comment,
    NAxis(u16),
    End,
    /*    AUTHOR,
        BITPIX,
        BLANK,
        BLOCKED,
        BSCALE,
        BUNIT,
        BZERO,
        CHECKSUM,
        COMMENT,
        CONTINUE,
        DATAMAX,
        DATAMIN,
        DATASUM,
        DATE,
        DATEOBS,
        END,
        EQUINOX,
        EXTEND,
        EXTNAME,
        EXTVER,
        FZALGOR,
        FZALGn(u16),
        FZTILELN,
        GCOUNT,
        GROUPS,
        HISTORY,
        INHERIT,
        INSTRUME,
        NAXIS,
        NAXISn(u16),
        OBJECT,
        OBSERVER,
        ORIGIN,
        PCOUNT,
        PTYPEn(u16),
        PSCALn(u16),
        PZEROn(u16),
        REFERENC,
        SIMPLE,
        TBCOL,
        TDISPn(u16),
        TDIMn(u16),
        TDMAXn(u16),
        TDMINn(u16),
        TELESCOP,
        TFIELDS,
        TFORM,
        THEAP,
        TLMAXn(u16),
        TLMINn(u16),
        TNULLn(u16),
        TTYPEn(u16),
        TSCALn(u16),
        TUNITn(u16),
        TZEROn(u16),
        XTENSION,
        ZBITPIX,
        ZBLOCKED,
        ZCMPTYPE,
        ZCTYPn,
        ZDATASUM,
        ZDITHER0,
        ZEXTEND,
        ZFORMn,
        ZGCOUNT,
        ZHECKSUM,
        ZMASKCMP,
        ZNAMEi(u16),
        ZNAXIS1,
        ZNAXIS2,
        ZIMAGE,
        ZPCOUNT,
        ZQUANTIZ,
        ZSIMPLE,
        ZTABLE,
        ZTENSION,
        ZTHEAP,
        ZTILEn(u16),
        ZTILELEN,
        ZVALi(u16),
    */
}

#[derive(Debug, PartialEq)]
pub enum ParseKeywordError {
    UnknownKeyword,
    NotANumber,
}

impl TryFrom<&[u8]> for Keyword {
    type Error = ParseKeywordError;

    fn try_from(i: &[u8]) -> Result<Self, Self::Error> {
        match i {
            b"COMMENT " => Ok(Keyword::Comment),
            b"SIMPLE  " => Ok(Keyword::Simple),
            b"BITPIX  " => Ok(Keyword::BitPix),
            b"NAXIS   " => Ok(Keyword::NAxis(0)),
            b"END" => Ok(Keyword::End),
            _ => Err(ParseKeywordError::UnknownKeyword),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ValueIndicator {
    EqualSpace,
    None,
    Unknown,
}

impl From<&[u8]> for ValueIndicator {
    fn from(i: &[u8]) -> Self {
        match i {
            b"= " => ValueIndicator::EqualSpace,
            b"  " => ValueIndicator::None,
            _ => ValueIndicator::Unknown,
        }
    }
}
