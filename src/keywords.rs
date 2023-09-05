#[derive(Debug, PartialEq)]
pub enum Keyword {
    Simple,
    BitPix,
    Comment,
    NAxis,
    NAxisn(u16),
    End,
    Unknown,
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

impl From<&[u8]> for Keyword {
    fn from(i: &[u8]) -> Self {
        match i {
            b"COMMENT " => Keyword::Comment,
            b"SIMPLE  " => Keyword::Simple,
            b"BITPIX  " => Keyword::BitPix,
            b"NAXIS   " => Keyword::NAxis,
            b"END" => Keyword::End,
            _ => Keyword::Unknown,
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
