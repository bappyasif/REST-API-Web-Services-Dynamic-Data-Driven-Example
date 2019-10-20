import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

usersList = WS.sendRequest(findTestObject('REST API Web Service/List Users'))

// Using JsonSlurper For Parsing Response And Extracting VAlue From Parsed Text.
def usersListJsonSlurper = new groovy.json.JsonSlurper()

def usersListResponseResult = usersListJsonSlurper.parseText(usersList.getResponseBodyContent())

def extractedUserResultValue = usersListResponseResult.data[1].first_name

println('User Found : ' + extractedUserResultValue)

GlobalVariable.USER_NAME = extractedUserResultValue

println('User Found : ' + GlobalVariable.USER_NAME)

WS.sendRequestAndVerify(findTestObject('REST API Web Service/Update User', [('userName') : GlobalVariable.USER_NAME]))

// Using Without Global Variable

WS.sendRequest(findTestObject('REST API Web Service/Update User', [('userName') : extractedUserResultValue]))

// Using Hardcoded Data Value

WS.sendRequest(findTestObject('REST API Web Service/Update User', [('userName') : "Bappy"]))

