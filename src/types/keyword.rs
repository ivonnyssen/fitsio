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
    Unknown([u8; 8]),
    Xtension,
    ZBitPix,
    ZBlocked,
    ZCTypn(u16),
    ZCmpType,
    ZDataSum,
    ZDither0,
    ZExtend,
    ZFormn(u16),
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
    ZVALi(u16),
    ZheckSum,
}

impl Keyword {
    pub fn new(prefix: &str, i: &[u8]) -> Self {
        let (_, number) = match prefix.len() > i.len() {
            true => {
                return Keyword::Unknown({
                    match i.try_into() {
                        Ok(i) => i,
                        Err(_) => return Keyword::Unknown(*b"KW ERROR"),
                    }
                })
            }
            false => i.split_at(prefix.len()),
        };

        match std::str::from_utf8(number).unwrap_or("").trim_end().parse() {
            Ok(n) => Self::combine(prefix, n),
            Err(_) => Keyword::Unknown({
                match i.try_into() {
                    Ok(i) => i,
                    Err(_) => return Keyword::Unknown(*b"KW ERROR"),
                }
            }),
        }
    }

    fn combine(prefix: &str, n: u16) -> Self {
        match prefix {
            "FZALG" => Keyword::FZALGn(n),
            "NAXIS" => Keyword::NAxisn(n),
            "PSCAL" => Keyword::PScaln(n),
            "PTYPE" => Keyword::PTypen(n),
            "PZERO" => Keyword::PZeron(n),
            "TBCOL" => Keyword::TBcoln(n),
            "TDMAX" => Keyword::TDMaxn(n),
            "TDMIN" => Keyword::TDMinn(n),
            "TDIM" => Keyword::TDimn(n),
            "TDISP" => Keyword::TDispn(n),
            "TFORM" => Keyword::TFormn(n),
            "TLMAX" => Keyword::TLMaxn(n),
            "TLMIN" => Keyword::TLMinn(n),
            "TNULL" => Keyword::TNulln(n),
            "TSCAL" => Keyword::TScaln(n),
            "TTYPE" => Keyword::TTypen(n),
            "TUNIT" => Keyword::TUnitn(n),
            "TZERO" => Keyword::TZeron(n),
            "ZCTYP" => Keyword::ZCTypn(n),
            "ZFORM" => Keyword::ZFormn(n),
            "ZNAME" => Keyword::ZNAMEi(n),
            "ZTILE" => Keyword::ZTilen(n),
            "ZVAL" => Keyword::ZVALi(n),
            _ => Keyword::Unknown({
                match prefix.as_bytes().try_into() {
                    Ok(i) => i,
                    Err(_) => return Keyword::Unknown(*b"KW ERROR"),
                }
            }),
        }
    }
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
            b"FZALG" => Keyword::new("FZALG", i),
            b"FZALGOR " => Keyword::FZAlgor,
            b"FZTILELN" => Keyword::FZTileLn,
            b"GCOUNT  " => Keyword::GCount,
            b"GROUPS  " => Keyword::Groups,
            b"HISTORY " => Keyword::History,
            b"INHERIT " => Keyword::Inherit,
            b"INSTRUME" => Keyword::Instrume,
            b"NAXIS   " => Keyword::NAxis,
            b"NAXIS" => Keyword::new("NAXIS", i),
            b"OBJECT  " => Keyword::Object,
            b"OBS     " => Keyword::Obs,
            b"OBSERVER" => Keyword::Observer,
            b"ORIGIN  " => Keyword::Origin,
            b"PCOUNT  " => Keyword::PCount,
            b"PSCAL" => Keyword::new("PSCAL", i),
            b"PTYPE" => Keyword::new("PTYPE", i),
            b"PZERO" => Keyword::new("PZERO", i),
            b"REFERENC" => Keyword::Referenc,
            b"SIMPLE  " => Keyword::Simple,
            b"TBCOL" => Keyword::new("TBCOL", i),
            b"TDMAX" => Keyword::new("TDMAX", i),
            b"TDMIN" => Keyword::new("TDMIN", i),
            b"TDIM" => Keyword::new("TDIM", i),
            b"TDISP" => Keyword::new("TDISP", i),
            b"TFORM" => Keyword::new("TFORM", i),
            b"THEAP   " => Keyword::THeap,
            b"TLMAX" => Keyword::new("TLMAX", i),
            b"TLMIN" => Keyword::new("TLMIN", i),
            b"TNULL" => Keyword::new("TNULL", i),
            b"TSCAL" => Keyword::new("TSCAL", i),
            b"TTYPE" => Keyword::new("TTYPE", i),
            b"TUNIT" => Keyword::new("TUNIT", i),
            b"TZERO" => Keyword::new("TZERO", i),
            b"TELESCOP" => Keyword::Telescop,
            b"TFIELDS " => Keyword::Tfields,
            b"XTENSION" => Keyword::Xtension,
            b"ZBITPIX " => Keyword::ZBitPix,
            b"ZBLOCKED" => Keyword::ZBlocked,
            b"ZCTYP" => Keyword::new("ZCTYP", i),
            b"ZCMPTYPE" => Keyword::ZCmpType,
            b"ZDATASUM" => Keyword::ZDataSum,
            b"ZDITHER0" => Keyword::ZDither0,
            b"ZEXTEND " => Keyword::ZExtend,
            b"ZFORM" => Keyword::new("ZFORM", i),
            b"ZGCOUNT " => Keyword::ZGCount,
            b"ZIMAGE  " => Keyword::ZImage,
            b"ZMASKCMP" => Keyword::ZMaskCmp,
            b"ZNAME" => Keyword::new("ZNAME", i),
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
            b"ZTILE" => Keyword::new("ZTILE", i),
            b"ZVAL" => Keyword::new("ZVAL", i),
            b"ZHECKSUM" => Keyword::ZheckSum,
            _ => Keyword::Unknown({
                match i.try_into() {
                    Ok(i) => i,
                    Err(_) => return Keyword::Unknown(*b"KW ERROR"),
                }
            }),
        }
    }
}

pub enum ParseKeywordError {
    NotANumber,
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

#[cfg(test)]
mod test {
    use nom::AsBytes;

    use super::*;
    #[test]
    fn test_keyword_new() {
        assert_eq!(Keyword::new("FZALG", b"FZALG1  "), Keyword::FZALGn(1));
        assert_eq!(Keyword::new("FZALG", b"FZALG999"), Keyword::FZALGn(999));
        assert_eq!(
            Keyword::new("FZALG", b"FZALG   "),
            Keyword::Unknown(b"FZALG   ".as_bytes().try_into().unwrap())
        );
        assert_eq!(
            Keyword::new("FZALG", b"FZALG###"),
            Keyword::Unknown(b"FZALG###".as_bytes().try_into().unwrap())
        );
        assert_eq!(
            Keyword::new("FZALG", b"FZA"),
            Keyword::Unknown(b"KW ERROR".as_bytes().try_into().unwrap())
        );
    }
    #[test]
    fn test_all_special_keywords() {
        assert_eq!(Keyword::new("NAXIS", b"NAXIS1  "), Keyword::NAxisn(1));
        assert_eq!(Keyword::new("PSCAL", b"PSCAL1  "), Keyword::PScaln(1));
        assert_eq!(Keyword::new("PTYPE", b"PTYPE1  "), Keyword::PTypen(1));
        assert_eq!(Keyword::new("PZERO", b"PZERO1  "), Keyword::PZeron(1));
        assert_eq!(Keyword::new("TBCOL", b"TBCOL1  "), Keyword::TBcoln(1));
        assert_eq!(Keyword::new("TDMAX", b"TDMAX1  "), Keyword::TDMaxn(1));
        assert_eq!(Keyword::new("TDMIN", b"TDMIN1  "), Keyword::TDMinn(1));
        assert_eq!(Keyword::new("TDIM", b"TDIM1   "), Keyword::TDimn(1));
        assert_eq!(Keyword::new("TDISP", b"TDISP1  "), Keyword::TDispn(1));
        assert_eq!(Keyword::new("TFORM", b"TFORM1  "), Keyword::TFormn(1));
        assert_eq!(Keyword::new("TLMAX", b"TLMAX1  "), Keyword::TLMaxn(1));
        assert_eq!(Keyword::new("TLMIN", b"TLMIN1  "), Keyword::TLMinn(1));
        assert_eq!(Keyword::new("TNULL", b"TNULL1  "), Keyword::TNulln(1));
        assert_eq!(Keyword::new("TSCAL", b"TSCAL1  "), Keyword::TScaln(1));
        assert_eq!(Keyword::new("TTYPE", b"TTYPE1  "), Keyword::TTypen(1));
        assert_eq!(Keyword::new("TUNIT", b"TUNIT1  "), Keyword::TUnitn(1));
        assert_eq!(Keyword::new("TZERO", b"TZERO1  "), Keyword::TZeron(1));
        assert_eq!(Keyword::new("ZCTYP", b"ZCTYP1  "), Keyword::ZCTypn(1));
        assert_eq!(Keyword::new("ZFORM", b"ZFORM1  "), Keyword::ZFormn(1));
        assert_eq!(Keyword::new("ZNAME", b"ZNAME1  "), Keyword::ZNAMEi(1));
        assert_eq!(Keyword::new("ZTILE", b"ZTILE1  "), Keyword::ZTilen(1));
        assert_eq!(Keyword::new("ZVAL", b"ZVAL1   "), Keyword::ZVALi(1));
    }

    #[test]
    fn test_all_keywords() {
        let keywords: Vec<(&[u8; 8], Keyword)> = Vec::from([
            ((b"        "), Keyword::Empty),
            ((b"AUTHOR  "), Keyword::Author),
            ((b"BSCALE  "), Keyword::BScale),
            ((b"BUNIT   "), Keyword::BUnit),
            ((b"BZERO   "), Keyword::BZero),
            ((b"BITPIX  "), Keyword::BitPix),
            ((b"BLANK   "), Keyword::Blank),
            ((b"BLOCKED "), Keyword::Blocked),
            ((b"CHECKSUM"), Keyword::CheckSum),
            ((b"COMMENT "), Keyword::Comment),
            ((b"CONTINUE"), Keyword::Continue),
            ((b"DATAMAX "), Keyword::DataMax),
            ((b"DATAMIN "), Keyword::DataMin),
            ((b"DATASUM "), Keyword::DataSum),
            ((b"DATE    "), Keyword::Date),
            ((b"DATE-OBS"), Keyword::DateObs),
            ((b"END     "), Keyword::End),
            ((b"EPOCH   "), Keyword::Epoch),
            ((b"EQUINOX "), Keyword::Equinox),
            ((b"EXTLEVEL"), Keyword::ExtLevel),
            ((b"EXTNAME "), Keyword::ExtName),
            ((b"EXTVER  "), Keyword::ExtVer),
            ((b"EXTEND  "), Keyword::Extend),
            ((b"FZALGOR "), Keyword::FZAlgor),
            ((b"FZTILELN"), Keyword::FZTileLn),
            ((b"GCOUNT  "), Keyword::GCount),
            ((b"GROUPS  "), Keyword::Groups),
            ((b"HISTORY "), Keyword::History),
            ((b"INHERIT "), Keyword::Inherit),
            ((b"INSTRUME"), Keyword::Instrume),
            ((b"NAXIS   "), Keyword::NAxis),
            ((b"OBJECT  "), Keyword::Object),
            ((b"OBS     "), Keyword::Obs),
            ((b"OBSERVER"), Keyword::Observer),
            ((b"ORIGIN  "), Keyword::Origin),
            ((b"PCOUNT  "), Keyword::PCount),
            ((b"REFERENC"), Keyword::Referenc),
            ((b"SIMPLE  "), Keyword::Simple),
            ((b"THEAP   "), Keyword::THeap),
            ((b"TELESCOP"), Keyword::Telescop),
            ((b"TFIELDS "), Keyword::Tfields),
            ((b"XTENSION"), Keyword::Xtension),
            ((b"ZBITPIX "), Keyword::ZBitPix),
            ((b"ZBLOCKED"), Keyword::ZBlocked),
            ((b"ZCMPTYPE"), Keyword::ZCmpType),
            ((b"ZDATASUM"), Keyword::ZDataSum),
            ((b"ZDITHER0"), Keyword::ZDither0),
            ((b"ZEXTEND "), Keyword::ZExtend),
            ((b"ZGCOUNT "), Keyword::ZGCount),
            ((b"ZIMAGE  "), Keyword::ZImage),
            ((b"ZMASKCMP"), Keyword::ZMaskCmp),
            ((b"ZNAXIS  "), Keyword::ZNaxis),
            ((b"ZNAXIS1 "), Keyword::ZNaxis1),
            ((b"ZNAXIS2 "), Keyword::ZNaxis2),
            ((b"ZPCOUNT "), Keyword::ZPCount),
            ((b"ZQUANTIZ"), Keyword::ZQuantiz),
            ((b"ZSIMPLE "), Keyword::ZSimple),
            ((b"ZTHEAP  "), Keyword::ZTHeap),
            ((b"ZTABLE  "), Keyword::ZTable),
            ((b"ZTENSION"), Keyword::ZTension),
            ((b"ZTILELEN"), Keyword::ZTileLen),
            ((b"ZHECKSUM"), Keyword::ZheckSum),
        ]);

        for (input, expected) in keywords {
            assert_eq!(Keyword::from(input.as_bytes()), expected);
        }
    }
}
