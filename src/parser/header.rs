use nom::{error::VerboseError, IResult};

use crate::types::keyword::Keyword;
use crate::types::KeywordRecord;

use super::keyword_record;

pub fn header(i: &[u8]) -> IResult<&[u8], Vec<KeywordRecord>, VerboseError<&[u8]>> {
    let mut last_block = false;
    let mut acc: Vec<KeywordRecord> = Vec::new();
    let mut input = i;
    while !last_block {
        for _ in 0..36 {
            match keyword_record::keyword_record(input) {
                Ok((i, record)) => match record.keyword() {
                    Keyword::End => {
                        //todo: rework trhe duplication below into a closure if I ever learn that
                        last_block = true;
                        input = i;
                        acc.push(record);
                    }
                    _ => {
                        input = i;
                        acc.push(record);
                    }
                },
                Err(e) => return Err(e),
            }
        }
    }
    Ok((input, acc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_header() {
        let mut input = String::from(
            r#"SIMPLE  =                    T / FITS STANDARD                                  "#,
        );
        input.push_str(
            r#"BITPIX  =                    8 / Character information                          "#,
        );
        input.push_str(
            r#"NAXIS   =                    0 / No image data array present                    "#,
        );
        input.push_str(
            r#"EXTEND  =                    T / There may be standard extensions               "#,
        );
        input.push_str(
            r#"DATE    = '31/10/97'           / Date file was written (dd/mm/yy) 19yy          "#,
        );
        input.push_str(
            r#"ORIGIN  = 'CEA/SSL UC Berkeley' / EUVE Science Archive                          "#,
        );
        input.push_str(
            r#"CREATOR = 'STWFITS '           / Fitsio version 11-May-1995                     "#,
        );
        input.push_str(
            r#"TELESCOP= 'EUVE    '           / Extreme Ultraviolet Explorer                   "#,
        );
        input.push_str(
            r#"INSTTYPE= 'DS/S    '           / Instrument type (DS/S, SCANNER)                "#,
        );
        input.push_str(
            r#"OBJECT  = 'NGC 4151'           / Name of observed object                        "#,
        );
        input.push_str(
            r#"RA_OBJ  =     182.635454000001 / R.A. of the object (degrees)                   "#,
        );
        input.push_str(
            r#"DEC_OBJ =     39.4057280000001 / Declination of the object (degrees)            "#,
        );
        input.push_str(
            r#"RA_PNT  =     182.988000000001 / R.A. of the pointing direction (degrees)       "#,
        );
        input.push_str(
            r#"DEC_PNT =              39.5477 / Declination of the pointing direction (degrees)"#,
        );
        input.push_str(
            r#"RA_PROC =     182.637910000001 / R.A. used to process data (degrees)            "#,
        );
        input.push_str(
            r#"DEC_PROC=             39.41343 / Declination used to process data (degrees)     "#,
        );
        input.push_str(
            r#"OBSERVER= 'A. A. Zdziarski'    / Original observing P.I. (EUVE = calibration)   "#,
        );
        input.push_str(
            r#"DATE-OBS= '30/04/97 GMT'       / Start date of observation (dd/mm/yy) 19yy      "#,
        );
        input.push_str(
            r#"TIME-OBS= '23:51:30 GMT'       / Start time of observation (hh:mm:ss GMT)       "#,
        );
        input.push_str(
            r#"DATE-END= '07/05/97 GMT'       / End date of observation (dd/mm/yy) 19yy        "#,
        );
        input.push_str(
            r#"TIME-END= '09:34:27 GMT'       / End time of observation (hh:mm:ss GMT)         "#,
        );
        input.push_str(
            r#"OBS_MODE= 'POINTING'           / Inertial pointing mode                         "#,
        );
        input.push_str(
            r#"DITHER  = 'NONE    '           / Spacecraft dither type (DITHERED, SPIRAL, NONE)"#,
        );
        input.push_str(
            r#"DETMODE = 'WSZ     '           / Detector position conversion mode (WSZ or XY)  "#,
        );
        input.push_str(
            r#"OFF-AXIS=                    T / Was this pointing done off-axis                "#,
        );
        input.push_str(
            r#"MOVING  =                    F / Did the source position vary during observation"#,
        );
        input.push_str(
            r#"DAYNIGHT= 'NIGHT   '           / Day/night data indicator (DAY, NIGHT, BOTH)    "#,
        );
        input.push_str(
            r#"VALIDTIM=      201378.81295777 / Amount of telemetry present (seconds)          "#,
        );
        input.push_str(
            r#"RA_UNIT = 'deg     '           / Units for Right Ascension                      "#,
        );
        input.push_str(
            r#"DEC_UNIT= 'deg     '           / Units for Declination                          "#,
        );
        input.push_str(
            r#"EQUINOX =                2000. / Coordinate equinox                             "#,
        );
        input.push_str(
            r#"RADECSYS= 'FK5     '           / Frame of reference of coordinates              "#,
        );
        input.push_str(
            r#"TIMESYS = 'MJD     '           / MJD = JD - 2400000.5                           "#,
        );
        input.push_str(
            r#"TIMEZERO=                   0. / No time offset required for EUVE event times   "#,
        );
        input.push_str(
            r#"TIMEUNIT= 's       '           / Units for TSTART, TSTOP, TIMEZERO              "#,
        );
        input.push_str(
            r#"CLOCKCOR= 'NO      '           / Not corrected to UT                            "#,
        );
        input.push_str(
            r#"TIMEREF = 'LOCAL   '           / No corrections applied (barycentric, etc.)     "#,
        );
        input.push_str(
            r#"TASSIGN = 'SATELLITE'          / Event times are assigned at the satellite      "#,
        );
        input.push_str(
            r#"TSTART  =     913161090.048001 / Time of start of observation (seconds)         "#,
        );
        input.push_str(
            r#"TSTOP   =     913714467.840001 / Time of end of observation (seconds)           "#,
        );
        input.push_str(
            r#"MJDREF  =               40000. / MJD of SC clock start, 24.00 May 1968          "#,
        );
        input.push_str(
            r#"EGOCSVER= 'egocs1.7.1'         / Software version used to produce this data     "#,
        );
        input.push_str(
            r#"REFVERS = 'egodata1.15.1'      / Reference calibration dataset version used     "#,
        );
        input.push_str(
            r#"COMMENT     ' '                                                                 "#,
        );
        input.push_str(
            r#"COMMENT     'This file is part of the EUVE Science Archive. It contains'        "#,
        );
        input.push_str(
            r#"COMMENT     'images and filter limits for one EUVE observation.'                "#,
        );
        input.push_str(
            r#"COMMENT     ' '                                                                 "#,
        );
        input.push_str(
            r#"COMMENT     'The EUVE Science Archive contains the science data from'           "#,
        );
        input.push_str(
            r#"COMMENT     'observations performed with the EUVE telescopes. It forms one'     "#,
        );
        input.push_str(
            r#"COMMENT     'part of the EUVE Permanent Archive. The other part of the'         "#,
        );
        input.push_str(
            r#"COMMENT     'permanent archive is the EUVE Telemetry Archive, which is a'       "#,
        );
        input.push_str(
            r#"COMMENT     'complete record of the raw telemetry from the EUVE mission.'       "#,
        );
        input.push_str(
            r#"COMMENT     ' '                                                                 "#,
        );
        input.push_str(
            r#"COMMENT     'For documentation of the contents of the EUVE Science Archive,'    "#,
        );
        input.push_str(
            r#"COMMENT     'see the "EUVE# Science Archive User's Guide".  The contents of'    "#,
        );
        input.push_str(
            r#"COMMENT     'the EUVE Telemetry Archive are described in the "EUVE'             "#,
        );
        input.push_str(
            r#"COMMENT     'Telemetry Archive User's Guide".'                                  "#,
        );
        input.push_str(
            r#"COMMENT     ' '                                                                 "#,
        );
        input.push_str(
            r#"COMMENT     'The EUVE Permanent Archive was produced by the Center for EUV'     "#,
        );
        input.push_str(
            r#"COMMENT     'Astrophysics, a division of UC Berkeley's Space Science'           "#,
        );
        input.push_str(
            r#"COMMENT     Laboratory.                                                         "#,
        );
        input.push_str(
            r#"COMMENT     ' '                                                                 "#,
        );
        input.push_str(
            r#"END                                                                             "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );
        input.push_str(
            r#"                                                                                "#,
        );

        let res = header(input.as_bytes());
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0, b"");
        assert_eq!(res.1.len(), 72);
    }

    //prop tests
    proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            let _ = header(s.as_bytes());
        }
    }
}
