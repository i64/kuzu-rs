use std::ffi::CStr;
use std::ffi::CString;

use crate::helper::into_cstr;
use crate::opaque_types::Opaque;

use super::database;
use super::opaque_types;

#[repr(u8)]
pub(crate) enum ConnectionTransactionMode {
    AutoCommit = 0,
    Manual = 1,
}

#[repr(C)]
pub struct QueryResult(Box<_QueryResult>);
pub(crate) type _QueryResult = Opaque<136>;

impl QueryResult {
    pub unsafe fn is_success(&self) -> bool {
        ffi::kuzu_main_QueryResult_isSuccess(self)
    }

    pub unsafe fn get_error_message(&mut self) {
        ffi::kuzu_main_QueryResult_getErrorMessage(self);
    }
}

#[repr(C)]
pub struct Connection(Box<Opaque<72>>);
pub(crate) type _Connection = Opaque<72>;

impl Connection {
    pub fn new(database: &mut database::Database) -> Self {
        unsafe {
            let mut _this = Box::new_uninit();
            ffi::kuzu_main_Connection_Connection(_this.as_mut_ptr(), database.0.as_mut());
            Self(_this.assume_init())
        }
    }

    pub unsafe fn query<S: AsRef<str>>(&mut self, query: S) -> *mut QueryResult {
        let (cstring_query, cstring_query_len) = into_cstr(query).unwrap();
        ffi::kuzu_main_Connection_query_c(self.0.as_mut(), cstring_query.as_ptr(), cstring_query_len)
    }
}

mod ffi {
    extern "C" {
        #[link_name = "\u{1}_ZN4kuzu4main10ConnectionC1EPNS0_8DatabaseE"]
        pub fn kuzu_main_Connection_Connection(
            this: *mut super::_Connection,
            database: *mut super::database::_Database,
        );

        #[link_name = "\u{1}_ZN4kuzu4main10Connection7query_cEPKcm"]
        pub fn kuzu_main_Connection_query_c(
            this: *mut super::_Connection,
            _query: *const ::std::os::raw::c_char,
            query_len: usize,
        ) -> *mut super::QueryResult;

        #[link_name = "\u{1}_ZNK4kuzu4main11QueryResult9isSuccessEv"]
        pub fn kuzu_main_QueryResult_isSuccess(this: *const super::QueryResult) -> bool;

        #[link_name = "\u{1}_ZNK4kuzu4main11QueryResult15getErrorMessageB5cxx11Ev"]
        pub fn kuzu_main_QueryResult_getErrorMessage(
            this: *const super::QueryResult,
        ) -> super::Opaque<16>;

        #[link_name = "\u{1}_ZNK4kuzu4main11QueryResult13getNumColumnsEv"]
        pub fn kuzu_main_QueryResult_getNumColumns(this: *const super::QueryResult) -> usize;

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult14getColumnNamesB5cxx11Ev"]
        pub fn kuzu_main_QueryResult_getColumnNames(
            this: *mut super::QueryResult,
        ) -> super::Opaque<24>;

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult18getColumnDataTypesEv"]
        pub fn kuzu_main_QueryResult_getColumnDataTypes(
            this: *mut super::QueryResult,
        ) -> super::Opaque<24>;

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult12getNumTuplesEv"]
        pub fn kuzu_main_QueryResult_getNumTuples(this: *mut super::QueryResult) -> u64;

        // #[link_name = "\u{1}_ZNK4kuzu4main11QueryResult15getQuerySummaryEv"]
        // pub fn kuzu_main_QueryResult_getQuerySummary(
        //     this: *const super::QueryResult,
        // ) -> *mut kuzu_main_QuerySummary;

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult18getColumnTypesInfoEv"]
        pub fn kuzu_main_QueryResult_getColumnTypesInfo(
            this: *mut super::QueryResult,
        ) -> super::Opaque<24>;

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult7hasNextEv"]
        pub fn kuzu_main_QueryResult_hasNext(this: *mut super::QueryResult) -> bool;

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult7getNextEv"]
        pub fn kuzu_main_QueryResult_getNext(this: *mut super::QueryResult) -> super::Opaque<16>;

        // #[link_name = "\u{1}_ZN4kuzu4main11QueryResult10writeToCSVERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEccc"]
        // pub fn kuzu_main_QueryResult_writeToCSV(
        //     this: *mut super::QueryResult,
        //     fileName: *const std_string,
        //     delimiter: ::std::os::raw::c_char,
        //     escapeCharacter: ::std::os::raw::c_char,
        //     newline: ::std::os::raw::c_char,
        // );

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResult13resetIteratorEv"]
        pub fn kuzu_main_QueryResult_resetIterator(this: *mut super::QueryResult);

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResultC1Ev"]
        pub fn kuzu_main_QueryResult_QueryResult(this: *mut super::QueryResult);

        // #[link_name = "\u{1}_ZN4kuzu4main11QueryResultC1ERKNS0_15PreparedSummaryE"]
        // pub fn kuzu_main_QueryResult_QueryResult1(
        //     this: *mut super::QueryResult,
        //     preparedSummary: *const kuzu_main_PreparedSummary,
        // );

        #[link_name = "\u{1}_ZN4kuzu4main11QueryResultD1Ev"]
        pub fn kuzu_main_QueryResult_QueryResult_destructor(this: *mut super::QueryResult);
    }
}
