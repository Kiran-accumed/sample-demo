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

WebUI.callTestCase(findTestCase('MOH-demo1/Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.mouseOver(findTestObject('Object Repository/Page_ROBINHome/span_Definitions'))

WebUI.click(findTestObject('Object Repository/Page_ROBINHome/span_Users'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_User_ui-button-icon-left ui-icon ui-c _75ba4a'))

WebUI.setText(findTestObject('Object Repository/Page_Users/input__AccumedFacilityUserCreateFormj_idt17_107b9e'), findTestData('New Data/Test Data').getValue(1,5))

WebUI.setText(findTestObject('Page_Users/input__AccumedFacilityUserCreateFormj_idt17_86d7a8'), findTestData('New Data/Test Data').getValue(2,5))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Users/input_Password_AccumedFacilityUserCreateFor_a75e5d'), 
    'C2WQ3vqOQ82tU3RlZ83ckw==')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Users/input_Confirm Password_AccumedFacilityUserC_10f9ca'), 
    'C2WQ3vqOQ82tU3RlZ83ckw==')

WebUI.click(findTestObject('Object Repository/Page_Users/input_The maximum Percentage of discount ()_23843f'))

WebUI.setText(findTestObject('Object Repository/Page_Users/input_Email List_AccumedFacilityUserCreateF_c8221a'), 'kiran@kiran.com')

WebUI.click(findTestObject('Object Repository/Page_Users/li_DATA-ENTRY'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_Test Admin1_ui-button-icon-left ui-ico_f2c47e'))

WebUI.click(findTestObject('Object Repository/Page_Users/li_COLLECTOR'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_Test Admin1_ui-button-icon-left ui-ico_f2c47e'))

WebUI.click(findTestObject('Object Repository/Page_Users/li_Admin-Test'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_Test Admin1_ui-button-icon-left ui-ico_f2c47e'))

WebUI.click(findTestObject('Object Repository/Page_Users/li_CASHIER ADMIN'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_Test Admin1_ui-button-icon-left ui-ico_f2c47e'))

WebUI.click(findTestObject('Object Repository/Page_Users/li_Manager'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_Test Admin1_ui-button-icon-left ui-ico_f2c47e'))

WebUI.click(findTestObject('Object Repository/Page_Users/span_Add_ui-button-icon-left ui-icon ui-c u_b5806f'))

//WebUI.click(findTestObject('Object Repository/Page_Users/span_Save'))

