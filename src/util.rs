use owo_colors::OwoColorize;

use crate::addresses::{Addresses, ScanExpr};
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

pub fn print_addrs(addrs: &mut Box<dyn Addresses>) {
    for (idx, (addr, old_val, new_val)) in addrs.get_vals_to_print().iter().enumerate() {
        if old_val == new_val {
            println!("{:3}: {:x}\t{}\t{}", idx, addr, old_val, new_val);
        } else {
            println!("{:3}: {:x}\t{}\t{}", idx, addr, old_val, new_val.red());
        }
    }
}
