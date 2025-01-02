use crate::addresses::ScanExpr;
use crate::commands::{FilterArgs, FilterOperator};

pub fn filter_args_to_scan_expr(filter_args: &FilterArgs) -> ScanExpr {
    match filter_args.operator {
        FilterOperator::Less => ScanExpr::Less(filter_args.operand.as_ref().unwrap().clone()),
        FilterOperator::LessEqual => {
            ScanExpr::LessEqual(filter_args.operand.as_ref().unwrap().clone())
        }
        FilterOperator::Greater => ScanExpr::Greater(filter_args.operand.as_ref().unwrap().clone()),
        FilterOperator::GreaterEqual => {
            ScanExpr::GreaterEqual(filter_args.operand.as_ref().unwrap().clone())
        }
        FilterOperator::Equal => ScanExpr::Equal(filter_args.operand.as_ref().unwrap().clone()),
        FilterOperator::NotEqual => {
            ScanExpr::NotEqual(filter_args.operand.as_ref().unwrap().clone())
        }
        FilterOperator::Changed => ScanExpr::Changed,
        FilterOperator::NotChanged => ScanExpr::NotChanged,
        FilterOperator::Unknown => ScanExpr::Unknown,
    }
}
