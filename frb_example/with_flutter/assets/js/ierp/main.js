/* eslint-disable space-before-function-paren */
/* eslint-disable no-empty */
/* eslint-disable no-multi-str */
/* eslint-disable spaced-comment */
/* eslint-disable no-undef */
/* eslint-disable eqeqeq */
/* eslint-disable dot-notation */
/* eslint-disable semi */
/* eslint-disable quotes */
/* eslint-disable prefer-const */
/* eslint-disable no-unused-vars */
function main() {
  let allMethods = {
    AmUnitTest: {
      BeforeGet: [
        "am/am_unit_test.js",
        "beforeGet"
      ]
    },
    AmAssetInstanceEv: {
      BeforePatch: [
        "am/am_asset_instance.js,am/am_asset_instance_calendar.js,am/am_asset_instance_meter.js",
        "beforePatch"
      ]
    },
    AmFaAssetV: {
      BeforePatch: [
        "am/am_asset_instance.js,am/am_asset_instance_calendar.js,am/am_asset_instance_meter.js",
        "beforePatch"
      ]
    },
    AmAssetGroup: {
      BeforePatch: [
        "am/am_asset_instance.js,am/am_asset_instance_calendar.js,am/am_asset_instance_meter.js",
        "beforePatch"
      ]
    },
    ArTransactionHeaderEv: {
      BeforePatch: [
        "ar/ar_transaction_header.js,shared/gl_journal_header_ev.js",
        "beforePatch"
      ]
    },
    ArPaymentHeaderEv: {
      BeforePatch: [
        "ar/ar_payment_header.js,shared/gl_journal_header_ev.js",
        "beforePatch"
      ]
    },
    ApPaymentHeaderEv: {
      BeforePatch: [
        "ap/ap_payment_header.js,shared/gl_journal_header_ev.js",
        "beforePatch"
      ]
    },
    ApTransactionHeaderEv: {
      BeforePatch: [
        "ap/ap_transaction_header.js,shared/gl_journal_header_ev.js",
        "beforePatch"
      ]
    },
    CstItemCostHeaderEv: {
      BeforePatch: ["cst/cst_item_cost_header_ev.js", "beforePatch"]
    },
    FaAssetEv: {
      BeforePatch: ["fa/fa_asset.js", "beforePatch"]
    },
    FpMdsHeaderEv: {
      BeforePatch: ["fp/fp_mds_header.js", "beforePatch"]
    },
    FpMrpHeaderEv: {
      BeforePatch: ["fp/fp_mrp_header.js", "beforePatch"]
    },
    FpMrpSupplyEv: {
      BeforePatch: ["fp/fp_mrp_supply.js", "beforePatch"]
    },
    FpMrpPlannedOrderV: {
      BeforePatch: ["fp/fp_mrp_supply.js", "beforePatch"]
    },
    FpMrpReleasePrV: {
      BeforePatch: ["fp/fp_mrp_supply.js", "beforePatch"]
    },
    FpMrpReleaseWoV: {
      BeforePatch: ["fp/fp_mrp_supply.js", "beforePatch"]
    },
    GlAvailablePeriodsV: {
      BeforePatch: ["gl/gl_period_ev.js", "beforePatch"]
    },
    GlJournalHeaderEv: {
      AfterGet: ["gl/gl_journal_header.js", "afterGet"],
      BeforePatch: ["gl/gl_journal_header.js", "beforePatch"]
    },
    GlJournalHeader: {
      AfterGet: ["gl/gl_journal_header.js", "afterGet"],
      BeforePatch: ["gl/gl_journal_header.js", "beforePatch"]
    },
    InvTransactionEv: {
      BeforePost: ["inv/inv_transaction.js", "beforePost"],
      BeforePatch: ["inv/inv_transaction.js", "beforePatch"]
    },
    HrLeaveEntitlementHeader: {
      BeforePatch: ["hr/hr_employee.js", "beforePatch"]
    },
    HrPayrollEv: {
      BeforePatch: ["hr/hr_payroll.js", "beforePatch"]
    },
    HrPayrollProcessV: {
      BeforePatch: ["hr/hr_payroll.js", "beforePatch"]
    },
    InvTransactionDocHeaderEv: {
      //  AfterPatch: ["inv/inv_transaction_doc_header.js", "afterPatch"],
      BeforePatch: [
        "inv/inv_transaction_doc_header.js,inv/gl_inv_transaction_doc_header.js,shared/gl_journal_header_ev.js",
        "beforePatch"
      ]
    },
    InvTransactionNewLineEv: {
      BeforeGet: ["inv/inv_transaction_doc_line.js", "beforeGet"]
    },
    InvItemEv: {
      AfterPost: ["inv/inv_item.js", "afterPost"],
      BeforePatch: ["inv/inv_item.js", "beforePatch"]
    },
    InvItemMasterEv: {
      BeforePatch: ["inv/inv_item_master.js", "beforePatch"]
    },
    InvAbcValHeaderEv: {
      BeforePatch: ["inv/inv_abc_val_header.js", "beforePatch"]
    },
    InvCountHeaderEv: {
      BeforePatch: ["inv/inv_count_header.js", "beforePatch"]
    },
    /*     PoRfqHeaderEv: {
      BeforePatch: ["po/purchasing_actions.js", "beforePatch"],
      AfterGet: ["po/purchasing_actions.js", "afterGet"],
    },
    PoRfqLineEv: {
      BeforePatch: ["po/purchasing_actions.js", "beforePatch"],
      AfterGet: ["po/purchasing_actions.js", "afterGet"],
    },
    PoQuoteHeaderEv: {
      BeforePatch: ["po/purchasing_actions.js", "beforePatch"],
      AfterGet: ["po/purchasing_actions.js", "afterGet"],
    },
    PoQuoteLineEv: {
      BeforePatch: ["po/purchasing_actions.js", "beforePatch"],
      AfterGet: ["po/purchasing_actions.js", "afterGet"],
    },
    PoRequisitionHeaderEv: {
      BeforePatch: ["po/purchasing_actions.js", "beforePatch"],
      AfterGet: ["po/purchasing_actions.js", "afterGet"],
    },
    PoRequisitionLineEv: {
      BeforePatch: ["po/purchasing_actions.js", "beforePatch"],
      AfterGet: ["po/purchasing_actions.js", "afterGet"],
    }, */
    PoRequisitionInterface: {
      BeforePatch: ["po/po_requisition.js", "beforePatch"]
    },
    PoHeaderEv: {
      BeforePatch: ["po/po_header.js", "beforePatch"]
      //  AfterGet: ["po/po_header.js", "afterGet"],
    },
    PoLineEv: {
      AfterPost: ["po/po_line.js", "afterPost"],
      AfterPatch: ["po/po_line.js", "afterPatch"]
    },
    PrjBillingDocHeaderEv: {
      BeforePatch: ["prj/prj_billing_doc_header_ev.js", "beforePatch"]
    },
    PrjBudgetHeaderEv: {
      BeforePatch: [
        "prj/prj_budget_header_ev.js,prj/prj_generate_draft_revenue.js",
        "beforePatch"
      ]
    },
    PrjExpenditureHeaderEv: {
      BeforePatch: ["prj/prj_expenditure_header_ev.js", "beforePatch"]
    },
    PrjProjectCostV: {
      BeforePatch: ["prj/prj_project_cost_v.js", "beforePatch"]
    },
    PrjProjectHeaderEv: {
      BeforePatch: [
        "prj/prj_revenue_doc_header_ev.js,prj/prj_generate_draft_revenue.js",
        "beforePatch"
      ]
    },
    PrjRevenueDocHeaderEv: {
      BeforePatch: [
        "prj/prj_revenue_doc_header_ev.js,prj/prj_generate_draft_revenue.js,shared/gl_journal_header_ev.js",
        "beforePatch"
      ]
    },
    SdSoHeaderEv: {
      BeforePatch: ["sd/so_header.js", "beforePatch"],
      //AfterGet: ["sd/so_header.js", "afterGet"],
      //BeforeGet: ["sd/so_header.js", "beforeGet"],
    },
    // SdDeliveryHeader: {
    //   BeforePatch: ["sd/sd_delivery_header.js", "beforePatch"],
    // },
    SdDeliveryLineEv: {
      BeforePatch: ["sd/sd_delivery_header.js", "beforePatch"]
    },
    WipWdHeaderEv: {
      AfterPost: ["wip/wd_header.js", "afterPost"]
    },
    WipWoInterface: {
      BeforePatch: ["wip/wo_header.js", "importWorkOrder"]
    },
    WipWoHeaderEv: {
      AfterPost: ["wip/wo_header.js", "afterPost"],
      AfterGet: ["wip/wo_header.js", "afterGet"],
      BeforePatch: ["wip/wo_header.js", "beforePatch"]
    },
    WipMoveTransactionEv: {
      BeforePost: ["wip/move_transaction.js", "beforePost"]
    },
    SdSoLineEv: {
      BeforePatch: ["sd/so_header.js", "beforePatch"]
    },
    RdSecUser: {
      afterGet: ["user/user.js", "removePassword"],
      BeforePatch: ["user/user.js", "validatePassword"],
      BeforePost: ["user/user.js", "validatePassword"]
    },
    RdSecRole: {
      BeforeGet: ["user/user.js", "validatePassword"],
      AfterGet: ["user/user.js", "validatePassword"],
      BeforePatch: ["user/user.js", "validatePassword"],
      AfterPatch: ["user/user.js", "validatePassword"],
      BeforePost: ["user/user.js", "validatePassword"],
      AfterPost: ["user/user.js", "validatePassword"],
      BeforeDelete: ["user/user.js", "validatePassword"],
      AfterDelete: ["user/user.js", "validatePassword"]
    },
    SysProgramHeaderV: {
      BeforePatch: ["sys/sys_program.js", "beforePatch"]
    }
  };

  return JSON.stringify(allMethods);
}
