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

WebUI.callTestCase(findTestCase('Robin-MOH/OP Dept/Robin_moh/login_case'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.mouseOver(findTestObject('Object Repository/Page_ROBINHome/span_Patient Journey Management'))

WebUI.click(findTestObject('Object Repository/Page_ROBINHome/span_Data Entry'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Create New Visit'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_New patient'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormmrn'), findTestData('New Data/Test Data').getValue(
        1, 12))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_National ID_AccumedPatientCreateForme_a5ef9f'), findTestData('New Data/Test Data').getValue(2, 12))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormMiddlename'), findTestData('New Data/Test Data').getValue(3, 12))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormpatientName'), findTestData('New Data/Test Data').getValue(4, 12))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormpatientSurname'), findTestData('New Data/Test Data').getValue(5, 12))

WebUI.click(findTestObject('Page_Data Entry/Element_label-gender'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Male'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormdateOfBirth2_input'), findTestData('New Data/Test Data').getValue(6, 12))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/a_23'))

WebUI.click(findTestObject('Page_Data Entry/span_Select One_ui-icon ui-icon-triangle-1-s ui-c'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Married'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Mobile_AccumedPatientCreateFormmobile'), findTestData(
        'New Data/Test Data').getValue(7, 12))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Email_AccumedPatientCreateFormemail'), findTestData(
        'New Data/Test Data').getValue(8, 12))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insert Patient'))

WebUI.mouseOver(findTestObject('Page_Data Entry/p_Patient has been added successfully'))

WebUI.getText(findTestObject('Page_Data Entry/p_Patient has been added successfully'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Data Entry/p_Patient has been added successfully'), 'Patient has been added successfully.')

WebUI.click(findTestObject('Page_Data Entry/Element_depart'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Cardiology'))

WebUI.click(findTestObject('Page_Data Entry/Element_endtype'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_HomeOther'))

WebUI.click(findTestObject('Page_Data Entry/Element_visit type'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_New'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label_Outpatient - patient not admitted and_1b549c'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Outpatient - patient not admitted and no_579973'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Clinician_InvoiceFormclinician_input'), 'm')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/td_AMAL  (R) ELSAID'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/div_Visit Details'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insert Visit'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_Data Entry/p_Visit has been created successfully'))

WebUI.closeBrowser()

