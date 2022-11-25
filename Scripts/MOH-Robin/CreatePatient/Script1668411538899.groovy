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

WebUI.callTestCase(findTestCase('MOH-Robin/Login'), [('Username') : '', ('Password') : ''], FailureHandling.STOP_ON_FAILURE)

WebUI.mouseOver(findTestObject('Object Repository/Page_ROBINHome/span_Patient Journey Management'))

WebUI.click(findTestObject('Object Repository/Page_ROBINHome/span_Data Entry'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Create New Visit'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_New patient'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormmrn'),findTestData('data1').getValue(1, 8))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_National ID_AccumedPatientCreateForme_a5ef9f'), findTestData('data1').getValue(5,8))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormpatientName'),findTestData('data1').getValue(2,6))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormMiddlename'),findTestData('data1').getValue(3,6))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormpatientSurname'),findTestData('data1').getValue(4,6))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label_'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Male'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormdateOfBirth2_input'),findTestData('data1').getValue(6,6))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Select One_ui-icon ui-icon-triangle-1-s ui-c'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Married'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Mobile_AccumedPatientCreateFormmobile'),findTestData('data1').getValue(7,6))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insert Patient'))

//WebUI.mouseOver(findTestObject('Object Repository/12/Page_Data Entry/p_Patient has been added successfully'))

// Msg= WebUI.getText(findTestObject('Object Repository/12/Page_Data Entry/p_Patient has been added successfully'))

// WebUI.verifyElementText(findTestObject('Object Repository/12/Page_Data Entry/p_Patient has been added successfully'), "Patient has been added successfully")

// System.out.println(Msg)

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label__1'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Cardiology'))


WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_End Date_InvoiceFormendDate_input'), '15/11/2022')

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Clinician_InvoiceFormclinician_input'), 's')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/td_ZEINAB OSMAN'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insert Visit'))

WebUI.click(findTestObject('Object Repository/12/Page_Data Entry/p_Visit has been created successfully'))


//Msg2= Visit has been created successfully

// WebUI.verifyElementText(findTestObject('Object Repository/12/Page_Data Entry/p_Visit has been created successfully'), Msg2)

// System.out.println(Msg2)
