#![recursion_limit = "256"]
fn main() {
    divan::main()
}

#[divan::bench]
fn serde_json() -> String {
    let data = serde_json::json! {{
      "description": "Service is a named abstraction of software service (for example, mysql) consisting of local port (for example 3306) that the proxy listens on, and the selector that determines which pods will answer requests sent through the proxy.",
      "properties": {
        "apiVersion": {
          "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources",
          "type": [
            "string",
            "null"
          ],
          "enum": [
            "v1"
          ]
        },
        "kind": {
          "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
          "type": [
            "string",
            "null"
          ],
          "enum": [
            "Service"
          ]
        },
        "metadata": {
          "description": "ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.",
          "properties": {
            "annotations": {
              "description": "Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations",
              "type": "object",
              "additionalProperties": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "clusterName": {
              "description": "The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.",
              "type": [
                "string",
                "null"
              ]
            },
            "creationTimestamp": {
              "type": [
                "string",
                "null"
              ],
              "format": "date-time"
            },
            "deletionGracePeriodSeconds": {
              "description": "Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.",
              "type": "integer",
              "format": "int64"
            },
            "deletionTimestamp": {
              "type": [
                "string",
                "null"
              ],
              "format": "date-time"
            },
            "finalizers": {
              "description": "Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed.",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "type": [
                  "string",
                  "null"
                ]
              },
              "x-kubernetes-patch-strategy": "merge"
            },
            "generateName": {
              "description": "GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.\n\nIf this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).\n\nApplied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#idempotency",
              "type": [
                "string",
                "null"
              ]
            },
            "generation": {
              "description": "A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.",
              "type": "integer",
              "format": "int64"
            },
            "initializers": {
              "description": "Initializers tracks the progress of initialization.",
              "required": [
                "pending"
              ],
              "properties": {
                "pending": {
                  "description": "Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.",
                  "type": "array",
                  "items": {
                    "description": "Initializer is information about an initializer that has not yet completed.",
                    "required": [
                      "name"
                    ],
                    "properties": {
                      "name": {
                        "description": "name of the process that is responsible for initializing this object.",
                        "type": "string"
                      }
                    }
                  },
                  "x-kubernetes-patch-merge-key": "name",
                  "x-kubernetes-patch-strategy": "merge"
                },
                "result": {
                  "description": "Status is a return value for calls that don't return other objects.",
                  "properties": {
                    "apiVersion": {
                      "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources",
                      "type": [
                        "string",
                        "null"
                      ],
                      "enum": [
                        "v1"
                      ]
                    },
                    "code": {
                      "description": "Suggested HTTP return code for this status, 0 if not set.",
                      "type": "integer",
                      "format": "int32"
                    },
                    "details": {
                      "description": "StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.",
                      "properties": {
                        "causes": {
                          "description": "The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.",
                          "type": [
                            "array",
                            "null"
                          ],
                          "items": {
                            "description": "StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.",
                            "properties": {
                              "field": {
                                "description": "The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.\n\nExamples:\n  \"name\" - the field \"name\" on the current resource\n  \"items[0].name\" - the field \"name\" on the first array entry in \"items\"",
                                "type": [
                                  "string",
                                  "null"
                                ]
                              },
                              "message": {
                                "description": "A human-readable description of the cause of the error.  This field may be presented as-is to a reader.",
                                "type": [
                                  "string",
                                  "null"
                                ]
                              },
                              "reason": {
                                "description": "A machine-readable description of the cause of the error. If this value is empty there is no information available.",
                                "type": [
                                  "string",
                                  "null"
                                ]
                              }
                            }
                          }
                        },
                        "group": {
                          "description": "The group attribute of the resource associated with the status StatusReason.",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "kind": {
                          "description": "The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "name": {
                          "description": "The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "retryAfterSeconds": {
                          "description": "If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.",
                          "type": "integer",
                          "format": "int32"
                        },
                        "uid": {
                          "description": "UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
                          "type": [
                            "string",
                            "null"
                          ]
                        }
                      }
                    },
                    "kind": {
                      "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
                      "type": [
                        "string",
                        "null"
                      ],
                      "enum": [
                        "Status"
                      ]
                    },
                    "message": {
                      "description": "A human-readable description of the status of this operation.",
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "metadata": {
                      "description": "ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.",
                      "properties": {
                        "continue": {
                          "description": "continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response.",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "resourceVersion": {
                          "description": "String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "selfLink": {
                          "description": "selfLink is a URL representing this object. Populated by the system. Read-only.",
                          "type": [
                            "string",
                            "null"
                          ]
                        }
                      }
                    },
                    "reason": {
                      "description": "A machine-readable description of why this operation is in the \"Failure\" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.",
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "status": {
                      "description": "Status of the operation. One of: \"Success\" or \"Failure\". More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status",
                      "type": [
                        "string",
                        "null"
                      ]
                    }
                  },
                  "x-kubernetes-group-version-kind": [
                    {
                      "group": "",
                      "kind": "Status",
                      "version": "v1"
                    }
                  ]
                }
              }
            },
            "labels": {
              "description": "Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels",
              "type": "object",
              "additionalProperties": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "name": {
              "description": "Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names",
              "type": [
                "string",
                "null"
              ]
            },
            "namespace": {
              "description": "Namespace defines the space within each name must be unique. An empty namespace is equivalent to the \"default\" namespace, but \"default\" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.\n\nMust be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces",
              "type": [
                "string",
                "null"
              ]
            },
            "ownerReferences": {
              "description": "List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "description": "OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.",
                "required": [
                  "apiVersion",
                  "kind",
                  "name",
                  "uid"
                ],
                "properties": {
                  "apiVersion": {
                    "description": "API version of the referent.",
                    "type": "string"
                  },
                  "blockOwnerDeletion": {
                    "description": "If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.",
                    "type": "boolean"
                  },
                  "controller": {
                    "description": "If true, this reference points to the managing controller.",
                    "type": "boolean"
                  },
                  "kind": {
                    "description": "Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
                    "type": "string"
                  },
                  "name": {
                    "description": "Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names",
                    "type": "string"
                  },
                  "uid": {
                    "description": "UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
                    "type": "string"
                  }
                }
              },
              "x-kubernetes-patch-merge-key": "uid",
              "x-kubernetes-patch-strategy": "merge"
            },
            "resourceVersion": {
              "description": "An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.\n\nPopulated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency",
              "type": [
                "string",
                "null"
              ]
            },
            "selfLink": {
              "description": "SelfLink is a URL representing this object. Populated by the system. Read-only.",
              "type": [
                "string",
                "null"
              ]
            },
            "uid": {
              "description": "UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.\n\nPopulated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
              "type": [
                "string",
                "null"
              ]
            }
          }
        },
        "spec": {
          "description": "ServiceSpec describes the attributes that a user creates on a service.",
          "properties": {
            "clusterIP": {
              "description": "clusterIP is the IP address of the service and is usually assigned randomly by the master. If an address is specified manually and is not in use by others, it will be allocated to the service; otherwise, creation of the service will fail. This field can not be changed through updates. Valid values are \"None\", empty string (\"\"), or a valid IP address. \"None\" can be specified for headless services when proxying is not required. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": [
                "string",
                "null"
              ]
            },
            "externalIPs": {
              "description": "externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "externalName": {
              "description": "externalName is the external reference that kubedns or equivalent will return as a CNAME record for this service. No proxying will be involved. Must be a valid RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires Type to be ExternalName.",
              "type": [
                "string",
                "null"
              ]
            },
            "externalTrafficPolicy": {
              "description": "externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. \"Local\" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. \"Cluster\" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.",
              "type": [
                "string",
                "null"
              ]
            },
            "healthCheckNodePort": {
              "description": "healthCheckNodePort specifies the healthcheck nodePort for the service. If not specified, HealthCheckNodePort is created by the service api backend with the allocated nodePort. Will use user-specified nodePort value if specified by the client. Only effects when Type is set to LoadBalancer and ExternalTrafficPolicy is set to Local.",
              "type": "integer",
              "format": "int32"
            },
            "loadBalancerIP": {
              "description": "Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature.",
              "type": [
                "string",
                "null"
              ]
            },
            "loadBalancerSourceRanges": {
              "description": "If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature.\" More info: https://kubernetes.io/docs/tasks/access-application-cluster/configure-cloud-provider-firewall/",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "ports": {
              "description": "The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "description": "ServicePort contains information on service's port.",
                "required": [
                  "port"
                ],
                "properties": {
                  "name": {
                    "description": "The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. This maps to the 'Name' field in EndpointPort objects. Optional if only one ServicePort is defined on this service.",
                    "type": [
                      "string",
                      "null"
                    ]
                  },
                  "nodePort": {
                    "description": "The port on each node on which this service is exposed when type=NodePort or LoadBalancer. Usually assigned by the system. If specified, it will be allocated to the service if unused or else creation of the service will fail. Default is to auto-allocate a port if the ServiceType of this Service requires one. More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport",
                    "type": "integer",
                    "format": "int32"
                  },
                  "port": {
                    "description": "The port that will be exposed by this service.",
                    "type": "integer",
                    "format": "int32"
                  },
                  "protocol": {
                    "description": "The IP protocol for this port. Supports \"TCP\" and \"UDP\". Default is TCP.",
                    "type": [
                      "string",
                      "null"
                    ]
                  },
                  "targetPort": {
                    "oneOf": [
                      {
                        "type": [
                          "string",
                          "null"
                        ]
                      },
                      {
                        "type": "integer"
                      }
                    ]
                  }
                }
              },
              "x-kubernetes-patch-merge-key": "port",
              "x-kubernetes-patch-strategy": "merge"
            },
            "publishNotReadyAddresses": {
              "description": "publishNotReadyAddresses, when set to true, indicates that DNS implementations must publish the notReadyAddresses of subsets for the Endpoints associated with the Service. The default value is false. The primary use case for setting this field is to use a StatefulSet's Headless Service to propagate SRV records for its Pods without respect to their readiness for purpose of peer discovery. This field will replace the service.alpha.kubernetes.io/tolerate-unready-endpoints when that annotation is deprecated and all clients have been converted to use this field.",
              "type": "boolean"
            },
            "selector": {
              "description": "Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/",
              "type": "object",
              "additionalProperties": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "sessionAffinity": {
              "description": "Supports \"ClientIP\" and \"None\". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": [
                "string",
                "null"
              ]
            },
            "sessionAffinityConfig": {
              "description": "SessionAffinityConfig represents the configurations of session affinity.",
              "properties": {
                "clientIP": {
                  "description": "ClientIPConfig represents the configurations of Client IP based session affinity.",
                  "properties": {
                    "timeoutSeconds": {
                      "description": "timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be >0 && <=86400(for 1 day) if ServiceAffinity == \"ClientIP\". Default value is 10800(for 3 hours).",
                      "type": "integer",
                      "format": "int32"
                    }
                  }
                }
              }
            },
            "type": {
              "description": "type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. \"ExternalName\" maps to the specified externalName. \"ClusterIP\" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object. If clusterIP is \"None\", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a stable IP. \"NodePort\" builds on ClusterIP and allocates a port on every node which routes to the clusterIP. \"LoadBalancer\" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the clusterIP. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services---service-types",
              "type": [
                "string",
                "null"
              ]
            }
          }
        },
        "status": {
          "description": "ServiceStatus represents the current status of a service.",
          "properties": {
            "loadBalancer": {
              "description": "LoadBalancerStatus represents the status of a load-balancer.",
              "properties": {
                "ingress": {
                  "description": "Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.",
                  "type": [
                    "array",
                    "null"
                  ],
                  "items": {
                    "description": "LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.",
                    "properties": {
                      "hostname": {
                        "description": "Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)",
                        "type": [
                          "string",
                          "null"
                        ]
                      },
                      "ip": {
                        "description": "IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)",
                        "type": [
                          "string",
                          "null"
                        ]
                      }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "x-kubernetes-group-version-kind": [
        {
          "group": "",
          "kind": "Service",
          "version": "v1"
        }
      ],
      "$schema": "http://json-schema.org/schema#",
      "type": "object"
    }};
    serde_json::to_string(&data).unwrap()
}

#[divan::bench]
fn typed_json() -> String {
    let data = typed_json::json! {{
      "description": "Service is a named abstraction of software service (for example, mysql) consisting of local port (for example 3306) that the proxy listens on, and the selector that determines which pods will answer requests sent through the proxy.",
      "properties": {
        "apiVersion": {
          "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources",
          "type": [
            "string",
            "null"
          ],
          "enum": [
            "v1"
          ]
        },
        "kind": {
          "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
          "type": [
            "string",
            "null"
          ],
          "enum": [
            "Service"
          ]
        },
        "metadata": {
          "description": "ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.",
          "properties": {
            "annotations": {
              "description": "Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations",
              "type": "object",
              "additionalProperties": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "clusterName": {
              "description": "The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.",
              "type": [
                "string",
                "null"
              ]
            },
            "creationTimestamp": {
              "type": [
                "string",
                "null"
              ],
              "format": "date-time"
            },
            "deletionGracePeriodSeconds": {
              "description": "Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.",
              "type": "integer",
              "format": "int64"
            },
            "deletionTimestamp": {
              "type": [
                "string",
                "null"
              ],
              "format": "date-time"
            },
            "finalizers": {
              "description": "Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed.",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "type": [
                  "string",
                  "null"
                ]
              },
              "x-kubernetes-patch-strategy": "merge"
            },
            "generateName": {
              "description": "GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.\n\nIf this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).\n\nApplied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#idempotency",
              "type": [
                "string",
                "null"
              ]
            },
            "generation": {
              "description": "A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.",
              "type": "integer",
              "format": "int64"
            },
            "initializers": {
              "description": "Initializers tracks the progress of initialization.",
              "required": [
                "pending"
              ],
              "properties": {
                "pending": {
                  "description": "Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.",
                  "type": "array",
                  "items": {
                    "description": "Initializer is information about an initializer that has not yet completed.",
                    "required": [
                      "name"
                    ],
                    "properties": {
                      "name": {
                        "description": "name of the process that is responsible for initializing this object.",
                        "type": "string"
                      }
                    }
                  },
                  "x-kubernetes-patch-merge-key": "name",
                  "x-kubernetes-patch-strategy": "merge"
                },
                "result": {
                  "description": "Status is a return value for calls that don't return other objects.",
                  "properties": {
                    "apiVersion": {
                      "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources",
                      "type": [
                        "string",
                        "null"
                      ],
                      "enum": [
                        "v1"
                      ]
                    },
                    "code": {
                      "description": "Suggested HTTP return code for this status, 0 if not set.",
                      "type": "integer",
                      "format": "int32"
                    },
                    "details": {
                      "description": "StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.",
                      "properties": {
                        "causes": {
                          "description": "The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.",
                          "type": [
                            "array",
                            "null"
                          ],
                          "items": {
                            "description": "StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.",
                            "properties": {
                              "field": {
                                "description": "The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.\n\nExamples:\n  \"name\" - the field \"name\" on the current resource\n  \"items[0].name\" - the field \"name\" on the first array entry in \"items\"",
                                "type": [
                                  "string",
                                  "null"
                                ]
                              },
                              "message": {
                                "description": "A human-readable description of the cause of the error.  This field may be presented as-is to a reader.",
                                "type": [
                                  "string",
                                  "null"
                                ]
                              },
                              "reason": {
                                "description": "A machine-readable description of the cause of the error. If this value is empty there is no information available.",
                                "type": [
                                  "string",
                                  "null"
                                ]
                              }
                            }
                          }
                        },
                        "group": {
                          "description": "The group attribute of the resource associated with the status StatusReason.",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "kind": {
                          "description": "The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "name": {
                          "description": "The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "retryAfterSeconds": {
                          "description": "If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.",
                          "type": "integer",
                          "format": "int32"
                        },
                        "uid": {
                          "description": "UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
                          "type": [
                            "string",
                            "null"
                          ]
                        }
                      }
                    },
                    "kind": {
                      "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
                      "type": [
                        "string",
                        "null"
                      ],
                      "enum": [
                        "Status"
                      ]
                    },
                    "message": {
                      "description": "A human-readable description of the status of this operation.",
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "metadata": {
                      "description": "ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.",
                      "properties": {
                        "continue": {
                          "description": "continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response.",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "resourceVersion": {
                          "description": "String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "selfLink": {
                          "description": "selfLink is a URL representing this object. Populated by the system. Read-only.",
                          "type": [
                            "string",
                            "null"
                          ]
                        }
                      }
                    },
                    "reason": {
                      "description": "A machine-readable description of why this operation is in the \"Failure\" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.",
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "status": {
                      "description": "Status of the operation. One of: \"Success\" or \"Failure\". More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status",
                      "type": [
                        "string",
                        "null"
                      ]
                    }
                  },
                  "x-kubernetes-group-version-kind": [
                    {
                      "group": "",
                      "kind": "Status",
                      "version": "v1"
                    }
                  ]
                }
              }
            },
            "labels": {
              "description": "Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels",
              "type": "object",
              "additionalProperties": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "name": {
              "description": "Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names",
              "type": [
                "string",
                "null"
              ]
            },
            "namespace": {
              "description": "Namespace defines the space within each name must be unique. An empty namespace is equivalent to the \"default\" namespace, but \"default\" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.\n\nMust be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces",
              "type": [
                "string",
                "null"
              ]
            },
            "ownerReferences": {
              "description": "List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "description": "OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.",
                "required": [
                  "apiVersion",
                  "kind",
                  "name",
                  "uid"
                ],
                "properties": {
                  "apiVersion": {
                    "description": "API version of the referent.",
                    "type": "string"
                  },
                  "blockOwnerDeletion": {
                    "description": "If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.",
                    "type": "boolean"
                  },
                  "controller": {
                    "description": "If true, this reference points to the managing controller.",
                    "type": "boolean"
                  },
                  "kind": {
                    "description": "Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
                    "type": "string"
                  },
                  "name": {
                    "description": "Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names",
                    "type": "string"
                  },
                  "uid": {
                    "description": "UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
                    "type": "string"
                  }
                }
              },
              "x-kubernetes-patch-merge-key": "uid",
              "x-kubernetes-patch-strategy": "merge"
            },
            "resourceVersion": {
              "description": "An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.\n\nPopulated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency",
              "type": [
                "string",
                "null"
              ]
            },
            "selfLink": {
              "description": "SelfLink is a URL representing this object. Populated by the system. Read-only.",
              "type": [
                "string",
                "null"
              ]
            },
            "uid": {
              "description": "UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.\n\nPopulated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
              "type": [
                "string",
                "null"
              ]
            }
          }
        },
        "spec": {
          "description": "ServiceSpec describes the attributes that a user creates on a service.",
          "properties": {
            "clusterIP": {
              "description": "clusterIP is the IP address of the service and is usually assigned randomly by the master. If an address is specified manually and is not in use by others, it will be allocated to the service; otherwise, creation of the service will fail. This field can not be changed through updates. Valid values are \"None\", empty string (\"\"), or a valid IP address. \"None\" can be specified for headless services when proxying is not required. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": [
                "string",
                "null"
              ]
            },
            "externalIPs": {
              "description": "externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "externalName": {
              "description": "externalName is the external reference that kubedns or equivalent will return as a CNAME record for this service. No proxying will be involved. Must be a valid RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires Type to be ExternalName.",
              "type": [
                "string",
                "null"
              ]
            },
            "externalTrafficPolicy": {
              "description": "externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. \"Local\" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. \"Cluster\" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.",
              "type": [
                "string",
                "null"
              ]
            },
            "healthCheckNodePort": {
              "description": "healthCheckNodePort specifies the healthcheck nodePort for the service. If not specified, HealthCheckNodePort is created by the service api backend with the allocated nodePort. Will use user-specified nodePort value if specified by the client. Only effects when Type is set to LoadBalancer and ExternalTrafficPolicy is set to Local.",
              "type": "integer",
              "format": "int32"
            },
            "loadBalancerIP": {
              "description": "Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature.",
              "type": [
                "string",
                "null"
              ]
            },
            "loadBalancerSourceRanges": {
              "description": "If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature.\" More info: https://kubernetes.io/docs/tasks/access-application-cluster/configure-cloud-provider-firewall/",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "ports": {
              "description": "The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": [
                "array",
                "null"
              ],
              "items": {
                "description": "ServicePort contains information on service's port.",
                "required": [
                  "port"
                ],
                "properties": {
                  "name": {
                    "description": "The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. This maps to the 'Name' field in EndpointPort objects. Optional if only one ServicePort is defined on this service.",
                    "type": [
                      "string",
                      "null"
                    ]
                  },
                  "nodePort": {
                    "description": "The port on each node on which this service is exposed when type=NodePort or LoadBalancer. Usually assigned by the system. If specified, it will be allocated to the service if unused or else creation of the service will fail. Default is to auto-allocate a port if the ServiceType of this Service requires one. More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport",
                    "type": "integer",
                    "format": "int32"
                  },
                  "port": {
                    "description": "The port that will be exposed by this service.",
                    "type": "integer",
                    "format": "int32"
                  },
                  "protocol": {
                    "description": "The IP protocol for this port. Supports \"TCP\" and \"UDP\". Default is TCP.",
                    "type": [
                      "string",
                      "null"
                    ]
                  },
                  "targetPort": {
                    "oneOf": [
                      {
                        "type": [
                          "string",
                          "null"
                        ]
                      },
                      {
                        "type": "integer"
                      }
                    ]
                  }
                }
              },
              "x-kubernetes-patch-merge-key": "port",
              "x-kubernetes-patch-strategy": "merge"
            },
            "publishNotReadyAddresses": {
              "description": "publishNotReadyAddresses, when set to true, indicates that DNS implementations must publish the notReadyAddresses of subsets for the Endpoints associated with the Service. The default value is false. The primary use case for setting this field is to use a StatefulSet's Headless Service to propagate SRV records for its Pods without respect to their readiness for purpose of peer discovery. This field will replace the service.alpha.kubernetes.io/tolerate-unready-endpoints when that annotation is deprecated and all clients have been converted to use this field.",
              "type": "boolean"
            },
            "selector": {
              "description": "Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/",
              "type": "object",
              "additionalProperties": {
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "sessionAffinity": {
              "description": "Supports \"ClientIP\" and \"None\". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": [
                "string",
                "null"
              ]
            },
            "sessionAffinityConfig": {
              "description": "SessionAffinityConfig represents the configurations of session affinity.",
              "properties": {
                "clientIP": {
                  "description": "ClientIPConfig represents the configurations of Client IP based session affinity.",
                  "properties": {
                    "timeoutSeconds": {
                      "description": "timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be >0 && <=86400(for 1 day) if ServiceAffinity == \"ClientIP\". Default value is 10800(for 3 hours).",
                      "type": "integer",
                      "format": "int32"
                    }
                  }
                }
              }
            },
            "type": {
              "description": "type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. \"ExternalName\" maps to the specified externalName. \"ClusterIP\" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object. If clusterIP is \"None\", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a stable IP. \"NodePort\" builds on ClusterIP and allocates a port on every node which routes to the clusterIP. \"LoadBalancer\" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the clusterIP. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services---service-types",
              "type": [
                "string",
                "null"
              ]
            }
          }
        },
        "status": {
          "description": "ServiceStatus represents the current status of a service.",
          "properties": {
            "loadBalancer": {
              "description": "LoadBalancerStatus represents the status of a load-balancer.",
              "properties": {
                "ingress": {
                  "description": "Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.",
                  "type": [
                    "array",
                    "null"
                  ],
                  "items": {
                    "description": "LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.",
                    "properties": {
                      "hostname": {
                        "description": "Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)",
                        "type": [
                          "string",
                          "null"
                        ]
                      },
                      "ip": {
                        "description": "IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)",
                        "type": [
                          "string",
                          "null"
                        ]
                      }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "x-kubernetes-group-version-kind": [
        {
          "group": "",
          "kind": "Service",
          "version": "v1"
        }
      ],
      "$schema": "http://json-schema.org/schema#",
      "type": "object"
    }};
    serde_json::to_string(&data).unwrap()
}
