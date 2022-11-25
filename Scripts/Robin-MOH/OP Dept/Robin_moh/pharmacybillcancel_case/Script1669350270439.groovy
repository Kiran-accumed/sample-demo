import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Robin_moh/login_case'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_ROBINHome/a_OP Billing'))

WebUI.click(findTestObject('Page_ROBINHome/a_OP Billing'))

WebUI.click(findTestObject('Page_ROBINHome/a_Pharmacy'))

WebUI.click(findTestObject('Page_ROBINHome/a_Pharmacy List'))

WebUI.click(findTestObject('Page_Pharmacy List/a_sahuyt sanyhja garam'))

WebUI.click(findTestObject('Page_Pharmacy List/Element_cancel'))

WebUI.setText(findTestObject('Page_Pharmacy List/textarea_You are cancelling bill ,please add note first_CancelBillFormj_idt1332'), 
    'not clear')

WebUI.click(findTestObject('Page_Pharmacy List/span_Cancel Bill'))

WebUI.click(findTestObject('Page_Pharmacy List/span_Close'))

WebUI.closeBrowser()

