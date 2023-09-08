#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
    Author,
    BScale,
    BUnit,
    BZero,
    BitPix,
    Blank,
    Blocked,
    CheckSum,
    Comment,
    Continue,
    DataMax,
    DataMin,
    DataSum,
    Date,
    DateObs,
    Empty,
    End,
    Epoch,
    Equinox,
    ExtLevel,
    ExtName,
    ExtVer,
    Extend,
    FZALGn(u16),
    FZAlgor,
    FZTileLn,
    GCount,
    Groups,
    History,
    Inherit,
    Instrume,
    NAxis,
    NAxisn(u16),
    Object,
    Obs,
    Observer,
    Origin,
    PCount,
    PScaln(u16),
    PTypen(u16),
    PZeron(u16),
    Referenc,
    Simple,
    TBcoln(u16),
    TDMaxn(u16),
    TDMinn(u16),
    TDimn(u16),
    TDispn(u16),
    TFormn(u16),
    THeap,
    TLMaxn(u16),
    TLMinn(u16),
    TNulln(u16),
    TScaln(u16),
    TTypen(u16),
    TUnitn(u16),
    TZeron(u16),
    Telescop,
    Tfields,
    Unknown,
    Xtension,
    ZBitPix,
    ZBlocked,
    ZCTypn,
    ZCmpType,
    ZDataSum,
    ZDither0,
    ZExtend,
    ZFormn,
    ZGCount,
    ZImage,
    ZMaskCmp,
    ZNAMEi(u16),
    ZNaxis,
    ZNaxis1,
    ZNaxis2,
    ZPCount,
    ZQuantiz,
    ZSimple,
    ZTHeap,
    ZTable,
    ZTension,
    ZTileLen,
    ZTilen(u16),
    ZVALi,
    ZheckSum,
}

impl From<&[u8]> for Keyword {
    fn from(i: &[u8]) -> Self {
        match i {
            b"        " => Keyword::Empty,
            b"AUTHOR  " => Keyword::Author,
            b"BSCALE  " => Keyword::BScale,
            b"BUNIT   " => Keyword::BUnit,
            b"BZERO   " => Keyword::BZero,
            b"BITPIX  " => Keyword::BitPix,
            b"BLANK   " => Keyword::Blank,
            b"BLOCKED " => Keyword::Blocked,
            b"CHECKSUM" => Keyword::CheckSum,
            b"COMMENT " => Keyword::Comment,
            b"CONTINUE" => Keyword::Continue,
            b"DATAMAX " => Keyword::DataMax,
            b"DATAMIN " => Keyword::DataMin,
            b"DATASUM " => Keyword::DataSum,
            b"DATE    " => Keyword::Date,
            b"DATE-OBS" => Keyword::DateObs,
            b"END     " => Keyword::End,
            b"EPOCH   " => Keyword::Epoch,
            b"EQUINOX " => Keyword::Equinox,
            b"EXTLEVEL" => Keyword::ExtLevel,
            b"EXTNAME " => Keyword::ExtName,
            b"EXTVER  " => Keyword::ExtVer,
            b"EXTEND  " => Keyword::Extend,
            b"FZALG{n} " => Keyword::FZALGn(n),
            b"FZALGOR " => Keyword::FZAlgor,
            b"FZTILELN" => Keyword::FZTileLn,
            b"GCOUNT  " => Keyword::GCount,
            b"GROUPS  " => Keyword::Groups,
            b"HISTORY " => Keyword::History,
            b"INHERIT " => Keyword::Inherit,
            b"INSTRUME" => Keyword::Instrume,
            b"NAXIS   " => Keyword::NAxis,
            b"NAXIS{u16}  " => Keyword::NAxisn(u16),
            b"OBJECT  " => Keyword::Object,
            b"OBS     " => Keyword::Obs,
            b"OBSERVER" => Keyword::Observer,
            b"ORIGIN  " => Keyword::Origin,
            b"PCOUNT  " => Keyword::PCount,
            b"PSCAL{u16} " => Keyword::PScaln(u16),
            b"PTYPE{u16} " => Keyword::PTypen(u16),
            b"PZERO{u16} " => Keyword::PZeron(u16),
            b"REFERENC" => Keyword::Referenc,
            b"SIMPLE  " => Keyword::Simple,
            b"TBCOL{u16} " => Keyword::TBcoln(u16),
            b"TDMAX{u16} " => Keyword::TDMaxn(u16),
            b"TDMIN{u16} " => Keyword::TDMinn(u16),
            b"TDIM{u16}  " => Keyword::TDimn(u16),
            b"TDISP{u16) " => Keyword::TDispn(u16),
            b"TFORM{u16}" => Keyword::TFormn(u16),
            b"THEAP   " => Keyword::THeap,
            b"TLMAX{u16} " => Keyword::TLMaxn(u16),
            b"TLMIN{u16} " => Keyword::TLMinn(u16),
            b"TNULL{u16} " => Keyword::TNulln(u16),
            b"SCAL{u16}  " => Keyword::TScaln(u16),
            b"TTYPE{u16} " => Keyword::TTypen(u16),
            b"TUNIT{u16} " => Keyword::TUnitn(u16),
            b"TZERO{u16} " => Keyword::TZeron(u16),
            b"TELESCOP" => Keyword::Telescop,
            b"TFIELDS " => Keyword::Tfields,
            _ => Keyword::Unknown,
            b"XTENSION" => Keyword::Xtension,
            b"ZBITPIX " => Keyword::ZBitPix,
            b"ZBLOCKED" => Keyword::ZBlocked,
            b"ZCTYP{us16} " => Keyword::ZCTypn,
            b"ZCMPTYP" => Keyword::ZCmpType,
            b"ZDATASUM" => Keyword::ZDataSum,
            b"ZDITHER0" => Keyword::ZDither0,
            b"ZEXTEND " => Keyword::ZExtend,
            b"ZFORM{u16} " => Keyword::ZFormn,
            b"ZGCOUNT " => Keyword::ZGCount,
            b"ZIMAGE  " => Keyword::ZImage,
            b"ZMASKCMP" => Keyword::ZMaskCmp,
            b"ZNAME{u16}" => Keyword::ZNAMEi(u16),
            b"ZNAXIS  " => Keyword::ZNaxis,
            b"ZNAXIS1 " => Keyword::ZNaxis1,
            b"ZNAXIS2 " => Keyword::ZNaxis2,
            b"ZPCOUNT " => Keyword::ZPCount,
            b"ZQUANTIZ" => Keyword::ZQuantiz,
            b"ZSIMPLE " => Keyword::ZSimple,
            b"ZTHEAP  " => Keyword::ZTHeap,
            b"ZTABLE  " => Keyword::ZTable,
            b"ZTENSION" => Keyword::ZTension,
            b"ZTILELEN" => Keyword::ZTileLen,
            b"ZTILE{u16} " => Keyword::ZTilen(u16),
            b"ZVAL{u16} " => Keyword::ZVALi,
            b"ZCHECKSUM" => Keyword::ZheckSum,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    CharacterString,
    ComplexFloat,
    ComplexInteger,
    ContinuedString,
    Date,
    Integer,
    Logical,
    Real,
    Unknown,
}
