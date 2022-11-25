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

WebUI.mouseOver(findTestObject('Object Repository/Page_ROBINHome/span_Patient Journey Management'))

WebUI.click(findTestObject('Object Repository/Page_ROBINHome/a_Data Entry'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Create New Visit'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_New patient'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormmrn'), '2184596378')

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_National ID_AccumedPatientCreateForme_a5ef9f'), '6789455145')

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormpatientName'), 'ghjhjfghgj')

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormMiddlename'), 'wrteterertytgfhjghj')

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormpatientSurname'), 'mkjjjwsdrfgth')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label_'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Male'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label_AFGHANISTAN'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Nationality_AccumedPatientCreateFormn_9d73c2'), 's')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_SAUDI ARABIA'))

//WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__AccumedPatientCreateFormdateOfBirth2_input'), '21/08/1987')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Select One_ui-icon ui-icon-triangle-1-s ui-c'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Married'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insert Patient'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label__1'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Emergency'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/label_Outpatient - patient not admitted and_1b549c'))

WebUI.click(findTestObject('Object Repository/Page_Data Entry/li_Hospital Inpatient - patient occupies in_dee41f'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_End Date_InvoiceFormendDate_input'), '26/11/2022')

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input_Clinician_InvoiceFormclinician_input'), 't')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/td_TARIQ ALYAMI'))

WebUI.setText(findTestObject('Object Repository/Page_Data Entry/input__InvoiceFormmrn'), 'APM-IP-00001337')

WebUI.click(findTestObject('Object Repository/Page_Data Entry/span_Insert Visit'))