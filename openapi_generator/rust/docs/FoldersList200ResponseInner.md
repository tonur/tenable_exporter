# FoldersList200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of the folder. | [optional]
**name** | Option<**String**> | The name of the folder. This value corresponds to the folder type as follows:  - main—My Scans  - trash—Trash -custom—user-defined string. | [optional]
**r#type** | Option<**String**> | The type of the folder: - main—Tenable-provided folder. Contains all scans that you create but do not assign to a custom folder, as well as any scans shared with you by other users. If you do not specify a scan folder when creating a scan, Tenable.io stores scans in this folder by default. This folder corresponds to the **My Scans** folder in the Tenable.io user interface.  - trash—Tenable-provided folder. This folder corresponds to the **Trash** folder in the Tenable.io user interface. It contains all scans that the current user has moved to the trash folder. After you move a scan to the trash folder, the scan remains in the trash folder until a user with at least Can Edit [64] scan permissions permanently deletes the scan.  - custom—User-created folder. Contains scans as assigned by the current user. You can create custom folders to meet your organizational needs. | [optional]
**default_tag** | Option<**i32**> | Indicates whether or not the folder is the default:  - 1—The folder is the default.  - 0—The folder is not the default.  The main folder is the default folder. You cannot change the default folder. | [optional]
**custom** | Option<**i32**> | Indicates whether or not the folder is a custom folder:  - 1—User-created folder. You can rename or delete this folder.  - 0—System-created folder. You cannot rename or delete this folder. | [optional]
**unread_count** | Option<**i32**> | The number of scans in the folder that the current user has not yet viewed in the Tenable.io user interface. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


