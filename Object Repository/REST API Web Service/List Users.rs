<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Users List, Get REST Request</description>
   <name>List Users</name>
   <tag></tag>
   <elementGuidId>6a26077c-1bed-4b72-9da7-7c2e3488da86</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${endPoint}/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endPoint</defaultValue>
      <description></description>
      <id>26b93558-1de0-4efa-9b77-929d71592c6d</id>
      <masked>false</masked>
      <name>endPoint</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'data[1].first_name', &quot;Lindsay&quot;)
//WS.verifyElementPropertyValue(response, 'data[1].last_name', &quot;Ferguson&quot;)
//WS.verifyElementPropertyValue(response, 'data[1].id', 8)
//WS.verifyElementPropertyValue(response, 'data[1].email', &quot;lindsay.ferguson@reqres.in&quot;)


WS.verifyElementPropertyValue(response, 'data[3].last_name', &quot;Fields&quot;)
WS.verifyElementPropertyValue(response, 'data[1].email', &quot;lindsay.ferguson@reqres.in&quot;)
WS.verifyElementPropertyValue(response, 'data[4].id', 11)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
