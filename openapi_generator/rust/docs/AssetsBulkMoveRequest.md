# AssetsBulkMoveRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | **String** | The UUID of the network currently associated with the assets. Use the [GET /networks](ref:networks-list) endpoint with the name attribute as filter to find the UUID of the network. | 
**destination** | **String** | The UUID of the network to associate with the specified assets. Use the [GET /networks](ref:networks-list) endpoint with the name filter to find the UUID of the network. | 
**targets** | **String** | The IPv4 addresses of the assets to move. The addresses can be represented as a comma-separated list, a range, or CIDR, for example `1.1.1.1, 2.2.2.2-2.2.2.200, 3.3.3.0/24`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


