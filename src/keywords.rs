#[derive(PartialEq, Debug)]
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

impl From<&[u8]> for Keyword {
    fn from(i: &[u8]) -> Self {
        match i {
            b"COMMENT " => Keyword::Comment,
            b"SIMPLE" => Keyword::Simple,
            b"BITPIX" => Keyword::BitPix,
            b"NAXIS" => Keyword::NAxis(0),
            b"END" => Keyword::End,
            _ => unimplemented!("unknown keyword"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PrimaryHDU {
    SIMPLE,
    BITPIX,
    NAXIS,
    NAXISn(u16),
    END,
}

#[derive(PartialEq, Debug)]
pub enum ConformingExtension {
    XTENSION,
    BITPIX,
    NAXIS,
    NAXISn(u16),
    PCOUNT,
    GCOUNT,
    END,
}

#[derive(PartialEq, Debug)]
pub enum ImageExtension {
    XTENSION,
    BITPIX,
    NAXIS,
    NAXISn(u16),
    PCOUNT, // =0
    GCOUNT, // =1
    END,
}

#[derive(PartialEq, Debug)]
pub enum ASCIITableExtension {
    XTENSION,
    BITPIX, //=8
    NAXIS,  //=2
    NAXIS1,
    NAXIS2,
    PCOUNT, //=0
    GCOUNT, //=1
    TFIELDS,
    TFORM(u16),
    TBCOL(u16),
    END,
}

#[derive(PartialEq, Debug)]
pub enum BinaryTableExtension {
    XTENSION,
    BITPIX, //=8
    NAXIS,  //=2
    NAXIS1,
    NAXIS2,
    PCOUNT,
    GCOUNT, // =1
    TFIELDS,
    TFORM(u16),
    END,
}

#[derive(PartialEq, Debug)]
pub enum CompressedImages {
    ZIMAGE, // =T
    ZBITPIX,
    ZNAXIS,
    ZNAXISn(u16),
    ZCMPTYPE,
}

#[derive(PartialEq, Debug)]
pub enum CompressedTables {
    ZTABLE, // =T
    ZNAXIS1,
    ZNAXIS2,
    ZPCOUNT,
    ZFORM(u16),
    ZCTYP(u16),
    ZTILELEN,
}

#[derive(PartialEq, Debug)]
pub enum RandomGroupsRecords {
    SIMPLE,
    BITPIX,
    NAXIS,
    NAXIS1, // =0
    NAXISn(u16),
    GROUPS, // =T
    PCOUNT,
    GCOUNT,
    END,
}
