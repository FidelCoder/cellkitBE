use crate::models::responses::ActionItem;

pub fn mvp_actions() -> Vec<ActionItem> {
    vec![
        ActionItem {
            id: "ckb.transfer",
            name: "CKB Transfer",
            description: "Build an unsigned CKB transfer transaction.",
            endpoint: "/api/actions/ckb-transfer/build",
            status: "mvp",
        },
        ActionItem {
            id: "xudt.transfer",
            name: "xUDT Transfer",
            description: "Build an unsigned xUDT transfer transaction.",
            endpoint: "/api/actions/xudt-transfer/build",
            status: "mvp",
        },
        ActionItem {
            id: "cell.consolidate",
            name: "Cell Consolidation",
            description: "Build an unsigned transaction that consolidates ordinary CKB cells.",
            endpoint: "/api/actions/cell-consolidation/build",
            status: "mvp",
        },
        ActionItem {
            id: "capacity.lock",
            name: "Capacity Lock",
            description:
                "Build an unsigned transaction that moves CKB capacity to a target lock address.",
            endpoint: "/api/actions/capacity-lock/build",
            status: "mvp",
        },
        ActionItem {
            id: "data_cell.create",
            name: "Data Cell Create",
            description: "Build an unsigned transaction that creates a CKB data cell.",
            endpoint: "/api/actions/data-cell-create/build",
            status: "mvp",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::mvp_actions;

    #[test]
    fn registry_returns_all_mvp_actions() {
        let ids: Vec<_> = mvp_actions().into_iter().map(|action| action.id).collect();
        assert_eq!(
            ids,
            vec![
                "ckb.transfer",
                "xudt.transfer",
                "cell.consolidate",
                "capacity.lock",
                "data_cell.create"
            ]
        );
    }
}
