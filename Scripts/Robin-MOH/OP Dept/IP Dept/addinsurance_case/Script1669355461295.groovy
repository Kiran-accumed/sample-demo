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

WebUI.callTestCase(findTestCase('Robin-MOH/OP Dept/IP Dept/Login'), [('Username') : '', ('Password') : ''], FailureHandling.STOP_ON_FAILURE)

WebUI.mouseOver(findTestObject('Object Repository/Page_ROBINHome/span_Patient Journey Management'))

WebUI.click(findTestObject('Object Repository/Page_ROBINHome/span_Data Entry'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Create New Visit Ammar_phWLFormsearch'), '284596314')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Search'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/a_fgMNDCFHS kjASDCFGG edgfrthju'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insurance Details'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Add Insurance'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientInsuranceCreateFormpat_cc52de'), '3466416315431')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label_'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Spouse_AccumedPatientInsuranceCreateF_20d039'), 't')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Tawuniya cooperative  Insurance Company'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_PolicyPlan_ui-button-icon-primary ui-i_6fb31b'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/td_CORPORATE'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Class_ui-button-icon-primary ui-icon u_1ed3f1'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/td_A'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_IP Admissions Only_ui-icon ui-icon-tri_fd308d'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_IP and OP Admissions'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Select One_ui-icon ui-icon-triangle-1-s ui-c'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_True'))

WebUI.click(findTestObject('Page_Data Entry/span_Save'))

WebUI.mouseOver(findTestObject('Object Repository/Page_Data Entry/p_Update patient successfully'))

Msg1 = WebUI.getText(findTestObject('Object Repository/Page_Data Entry/p_Update patient successfully'))

System.out.print(Msg1)

WebUI.verifyElementText(findTestObject('Object Repository/Page_Data Entry/p_Update patient successfully'), 'Update patient successfully.')

WebUI.click(findTestObject('span/Page_Data Entry/span_Save'))

WebUI.mouseOver(findTestObject('Object Repository/Page_Data Entry/p_Update patient successfully'))

Msg2 = WebUI.getText(findTestObject('Object Repository/Page_Data Entry/p_Update patient successfully'))

System.out.print(Msg2)

WebUI.verifyElementText(findTestObject('Object Repository/Page_Data Entry/p_Update patient successfully'), 'Update patient successfully.')

