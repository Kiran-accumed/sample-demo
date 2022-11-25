<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_OPENING VISIT IN PROGRESS.AlertOK     _c1c05f</name>
   <tag></tag>
   <elementGuidId>07a0241b-1f98-43cc-a2ca-7fc986e0e1c2</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body.ui-layout-container</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>928fd214-500c-4ace-b4ee-f0d81dbb9972</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>ui-layout-container</value>
      <webElementGuid>0a4ea04c-d6f1-4491-a1da-379e81248705</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>











OPENING VISIT IN PROGRESS...



Alert





















OK







    
        var ID, TYPE;

        function unloadPage() {
            console.log('222');
            unload();
        }
        function unload() {
            makeRequest( );
        }
        window.onbeforeunload = unloadPage;



        var facilityLicense;
        function loadRtlStyle(hrefpath) {
        }

        function removeRtlStyle() {
        }
        function applyEnglishLanguage() {
            deleteLang();
            setLang(&quot;lang&quot;, &quot;en&quot;, 365);
        }
        function applyArabicLanguage() {
            deleteLang();
            setLang(&quot;lang&quot;, &quot;ar&quot;, 365);
        }
        function setLang(cname, cvalue, exdays) {
            var d = new Date();
            d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
            var expires = &quot;expires=&quot; + d.toUTCString();
            document.cookie = cname + &quot;=&quot; + cvalue + &quot;;&quot; + expires + &quot;;path=/&quot;;
        }
        function getCurrentLangUpdate(cname) {
            var name = cname + &quot;=&quot;;
            var decodedCookie = decodeURIComponent(document.cookie);
            var ca = decodedCookie.split(';');
            for (var i = 0; i != ca.length; i++) {
                var c = ca[i];
                while (c.charAt(0) == ' ') {
                    c = c.substring(1);
                }
                if (c.indexOf(name) == 0) {
                    return c.substring(name.length, c.length);
                }
            }
            return &quot;&quot;;
        }
        function getCurrentLang(cname) {

            var name = cname + &quot;=&quot;;
            var decodedCookie = decodeURIComponent(document.cookie);
            var ca = decodedCookie.split(';');
            for (var i = 0; i != ca.length; i++) {
                var c = ca[i];
                while (c.charAt(0) == ' ') {
                    c = c.substring(1);
                }
                if (c.indexOf(name) == 0) {
                    return c.substring(name.length, c.length);
                }
            }
        }

        function deleteLang() {
            document.cookie = &quot;lang=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;&quot;;
        }

//           jQuery(function ($) {
//               if (getCurrentLang('lang') === 'ar') {
//                   var hrefCSS = '../resources/css/bootstrap-rtl.css';
//                   loadRtlStyle(hrefCSS);
//               }
//            });
        var refreshArabicCss = false;
        function checkArabic() {
            if (getCurrentLang('lang') == 'ar') {

                let headelements = document.getElementsByTagName('head');
                for (var i = 0; i != headelements.length; i++) {
                    let headelement = document.getElementsByTagName('head')[i];
                    if (true) {
                        var x1 = document.getElementsByClassName('header');
                        if (x1[0].children[0].innerHTML != 'الرئيسية')
                        {
//      
                            setTimeout(refreshPage, 1000);
                        }

                    }
                }
            }
        }



        $(document).ready(function () {
        });
        function refreshPage() {
            location.reload();
        }
    

    
    
    
    


myCommand = function() {PrimeFaces.ab({s:&quot;j_idt37:j_idt38&quot;,f:&quot;j_idt37&quot;,pa:arguments[0]});}



HomePatient Journey Management Data Entry View VisitsCheck EligibilityOut Patient AuthorizationsIn Patient AuthorizationsOP BillingServicesOP ReceiptsER ReceiptsPharmacyPharmacy ListPharmacy POSIP BillingIP BillsIP Receipts Collections Daily collection Collection Daily Report Advanced Collection Report Deliver money to cashier administrator Settlement Receivable Advance CollectionRefund AdvanceRefund Receipts Definitions Patient Users Roles ContractUpload prices Manage Contracts Manage Price List Append Price Manage Payers Companies ProfilesClinicians Codes Mapping HL7 Log Back Office Management Batch Coding Auditing Submission Claim Submission Batch Submission Payment Management Upload Remittance Denial Management MIS Reports Visit Report









                    

                    
                        
                        
                    
administrator (King Salman Abdulaziz Hospital -Riyadh)HospitalKing Salman Abdulaziz Hospital -Riyadh                                                 
                                            Al-Jubail General Hospital - Jubail                                                 
                                            Ohod General Hospital -Medinah                                                 
                                            Al Yamamah Hospital - Riyadh                                                 
                                            Khamis Mushayt Maternity and children Hospital                                                 
                                            Al-Qrayat General Hospital                                                 
                                            Rabigh Hospital                                                 
                                            King Khalid Hospital-Tabouk                                                 
                                            Maternity &amp; Children Hospital – Tabuk                                                 
                                            Maternity and Children Hospital-Najran                                                 
                                            King Khaled Hospital  -Hail                                                 
                                            Maternity and Children Hospital -Hail                                                 
                                            Sabya General Hospital - Sabya                                                 
                                            Dubaa General Hospital-Tabouk                                                 
                                            King Fahad Hospital -Jeddah                                                 
                                            King Abdul Aziz Hospital and Oncology- Jeddah                                                 
                                            Maternity &amp; Children Hospital - Aljouf                                                 
                                            Prince Moteb bin Abdulaziz Hospital- Aljouf                                                 
                                            Tarif General Hospital - Tarif                                                 
                                            King Fahad Hospital - Al Baha                                                 
                                            King Khaled Hospital -Najran                                                 
                                            King Fahad Central Hospital - Hafouf                                                 
                                            King Khaled Hospital - Hafer Al Baten                                                 
                                            King Faisal Hospital-Taif                                                 
                                            Children Hospital in Taif                                                 
                                            King Abdullah Hospital - Beshah                                                 
                                            ALIMAN GENERAL HOSPITAL                                                 
                                            اللغة العربيةEnglishUser InfoLOGOUT







HomePatient Journey ManagementData Entryfocus = function() {PrimeFaces.ab({s:&quot;mainForm:j_idt169&quot;,f:&quot;mainForm&quot;,u:&quot;mainForm:facilityLicense&quot;,onco:function(xhr,status,args){checkChangeLicense();;},pa:arguments[0]});};$(function() {focus();});



$(function(){PrimeFaces.cw(&quot;Poll&quot;,&quot;checkModifiedChangedPoll&quot;,{id:&quot;phWLForm:checkModifiedChangedPoll&quot;,frequency:30000,autoStart:true,fn:function(){PrimeFaces.ab({s:&quot;phWLForm:checkModifiedChangedPoll&quot;,f:&quot;phWLForm&quot;,g:false});}});});onload = function() {PrimeFaces.ab({s:&quot;phWLForm:j_idt170&quot;,f:&quot;phWLForm&quot;,pa:arguments[0]});};$(function() {onload();});


Create New Visit Ammar

Search
Rerouted from Coding stage.








                                    Create New VisitExportui-button
                                    
                                    

                                       FP1NE Rows Per Page20304050607080
                                PatientFilter by PatientMRNFilter by MRNVisit NoFilter by Visit NoVisit DateAdmission Date 
                                      
                                    Visit DateDischarge Date 
                                      
                                    National IDFilter by National IDPhoneDepartmentFilter by DepartmentDoctorDay Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.FilterDepartmentAllergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryFilterDoctorFilter by DoctorPayment TypeAllInsuranceCashAllBill TypeAllNo servicesBilledPartially BilledRefundedNot BilledAllEligibility StatusPayer... SAAUDI(The Mediterranean &amp; Gulf Cooperative Insurance &amp; Reinsurance Company (MedGulf???????????Accumed Comany123132020-NACIG - TCSACIG- MEDNETAL ETIHADAl Sagr Cooperative  Insurance CompanyALBAHAALFA131ALJazira TakafulALJUMAIH COallamAllianz Saudi Fransi Insurance CompanyAlrajhi Takaful for Cooperative InsuranceAMANA INSURANCEARABIA InsuranceARABIAN SHEILDARAMCOAXA Cooperative Insurance CompanybandrbupaBupaBUPA Arabia for Cooperative InsuranceBURUJDate SeptEnvironmentETIHADfgfdgfvdgcfhgnvbGENERAL 2021GENERAL-2020-JgggGLOBEMEDGLOBEMED SAUDI ( GMS )GLOBEMED SAUDI ( GMS )GM-AMINAHGOSIGULF GENERAL - MEDNETGULF GENERAL - TCSGulf general Insurance CompanyGulf unionGulf Union Cooperative Insurance CompanytesthghgfdfdHKK Coinsurance company nameinsuranceTestJPJubil InsKHAMISKSAKSAKSA001Malath Cooperative Insurance &amp; Reinsurance CompanymaxmazMEDNETMOUNIRMS-NIZIHAmyNextcareRTRSAGR - MEDNETSALAMA INSURANCEsaudiSaudi Arabian cooperative Insurance Company (Saico)Saudi Enaya Cooperative InsuranceSaudi United  Cooperative Insurance  (WALA'A)sdfsSelf Pay - CashsolidaritysosoSPSRSYTawuniya cooperative  Insurance CompanyTawuniya cooperative  Insurance CompanyffgTCStest2020testAL RAJHItestAL SAGR - NEXTCAREtestMalathtestMalath CooperativetestMedGulftestRAJHIAAtestSAICOtestSAUDI ENAYAtestSGH STAFF - TAWUNIYAtestTAWUNIYAtestTawuniya cooperative Insurance CompanytestWALAAtestWALAA - NEXTCAREToday-27-07TOKIO MARINETOTAL CARE TOTAL CARE SAUDI (TCS)TRADE UNIONUAEUCAUNSPECIFIEDyamamahyt...DepartmentCreated ByLast Update ByDetailsfgMNDCFHS kjASDCFGG edgfrthju284596314APM-IP-000013525/11/202225/11/202225/11/202225/11/2022788491249594758126549521351 | No Bed + No Emergency RoomCardiologyDr. MUATH ALMUQHIMCashNo servicesPendingKSP001|Self Pay - Cash952135ui-button




Claim Audit#Login UserAudit DateClaim IDAction TypeAction IdentifierUpdated FieldsHIS Msg IdHIS Msg ActionHIS Msg TypeNo records found.FPNE    

YesNoCancel Prescription





Are you sure to cancel the prescription?


Please write down the reason for cancellation







                            Cancel AuthorizationClose       
                            
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfCondition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedPackageTabletOther ServicePrincipalSecondaryPBM Details


Accumed Patient InfoPatientNational IDDOB / AgeGenderMember IDMRNPayerId PackageCard Type:InsurancePrescription DateDoctor|


Diagnosis 


Diagnosis Date

Diagnosis Type
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfadmitting
Condition Onset Flag
Condition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedCondition not noted as arising during the episode of admitted patient care






Diagnosis 

Add Diagnosis



#Diagnosis TypeDiagnosis CodeActionsNo record found.



ui-buttonMedical Services


Code
Date
DrugPriceType
Qty
Price 
Total Price
Clinician:
Intervention Type
Sequence
daySupply
timeUnit
 
 




PackageTabletPackage




Other ServicePrincipalSecondaryOther Service



Add Service



#Service codeservice discerption 
                        App QtyPrice (SAR)Gross Amt (SAR)                                       PT + VATNet + VATActionsNotes/ObsTotal-SAR0.000.000.00No record found.

                        
                           FPNE 
                    Request Approval Request
                            Send Approval Request
                            Service codeAlert TypeQuan.NetClinicianNo record found.FPNERows Per Page1020304050FPNE#Response ResultAuthorization TypeTransaction DateNo records found.





                Send Request AuthorizationClose
                
Response Observations


(1 of 1) FPNENo records found.
Dispens Result





Priscreption failed to be dispensed,Sending submission file has been failed




Cancel Prescription





Are you sure to cancel the prescription?


Please write down the reason for cancellation







                Close       
                
Visit DetailsPatient &amp; Encounter InfoDiagnosis &amp; Consultation CodesCheck RulesVisit Notes / AttachmentsVital Signs DetailsEligibility History



ui-buttonsomeCommand = function() {PrimeFaces.ab({s:&quot;InvoiceForm:j_idt730&quot;,f:&quot;InvoiceForm&quot;,u:&quot;InvoiceForm:checkRequestsIntervalPoll InvoiceForm:editPatientButton InvoiceForm:patient InvoiceForm:cardInfo2 InvoiceForm:zoomPanel InvoiceForm:zoomPanel1 InvoiceForm:eligibilityRequests actionsFormInvoice InvoiceForm:copaymentBTN InvoiceForm:deductableBTN InvoiceForm:patientInsuranceGroup&quot;,pa:arguments[0]});}ui-buttonSearch Patient






New patient

                    
           
                    
Treatment Approval











Edit patient
LOG
Copayment
Deductible
Insurance Details






PatientfgMNDCFHS kjASDCFGG edgfrthjuNational ID7884912495NationalityAFGHANISTANDOB / Age15/08/1989, 33Y 3M 10DGenderMaleMarital:MarriedMember IDNO CARDMRN284596314Payment Type:Cash



Patient Audit#Login User Audit DateAction TypeAction IdentifierUpdated FieldsHIS Msg IdHIS Msg ActionHIS Msg TypeNo records found.FPNE


Rotate Left
Rotate Right
Scale +
Scale -
Download
















Rotate Left
Rotate Right
Scale +
Scale -
Download













FPNE#Response ResultEncounter TypeTransaction DateNo records found.#MeaningValueCeilingNo records found.ValueMeaningValueCeilingNo records found.Encounter


Visit Id*

Department 
...Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryCardiology
Start Date:*



End Date:

Encounter Type:*



...Day Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.Hospital Inpatient - patient occupies inpatient bed and stays overnight




Start Type:*
...Elective waiting list admission government free SchemeElective waiting list admission insurance coverage SchemeElective waiting list admission self-payment SchemeEmergency Admission by insurance companyEmergency Admission by referral from general government hospitalEmergency Admission by referral from government primary healthcare centerEmergency Admission by referral from private hospitalEmergency Admission by referral from private primary healthcare centerEmergency Admission from hospital EREmergency Admission from hospital outpatientEmergency Maternity Birth AdmissionImmediate AdmissionOthersPlanned Maternity Birth AdmissionUnknownOthers


End Type:
...Discharge /Transfer to Other Health Care AccommodationDischarge /Transfer to a Residential Ageing ServiceDischarge/Transfer to a Psychiatric HospitalDischarge/transfer to an Acute HospitalHome/OtherStatistical Discharge-Type Change...
Service Event Type:
Initial client service eventSubsequent client service eventInitial client service event
Care Type::
Acute CareBoarderComplex MaintenanceGeneral MaintenancePalliativeRehabilitationAcute Care


Visit Type
...Follow upNewReferralRefillWalk In...
Clinician:








Specialty  :
GENERAL PRACTICE GENERAL PRACTICE


Lenght of stay: 




Emergency Details


Dept Disposition:
Admitted to this hospitalDead on arrivalDid not wait to be attended by a health care professionalDied in EDLeft at own risk after being attended by a health care professional, but before the non-admitted patient ED service episode was completedNon-admitted patient ED episode completed -referred to another hospital for admission;Non-admitted patient ED episode completed-departed without being admitted or referred to another hospitalRegistered, advised of another health care service, and left the EDwithout being attended by a health care professionalAdmitted to this hospital
Service Date:



Waiting Time:

Arrival Code:



Triag Category
Immediate resuscitationNon-UrgentStandard ERUrgentVery UrgentImmediate resuscitation



Advanced


Speciality

Ward

Room



Bed

Cause Of Death

Past History



Chief Complaint

Physical Exam

Main Symptoms



Discharge Notes






    
        .apply {
            background-color: #fca752 !important;
            background-image: none !important;
            color: #000000 !important;
        }
    
    
    
    
    
    
    
        .cheap {
            background-color: #54a90a !important;

        }
    
    
        .pend {
            background-color: #3197d6 !important;

        }
    
    
        .rej {
            background-color: #e00b0b !important;

        }
    



Diagnosis 


Diagnosis Date

Diagnosis Type
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfadmitting
Condition Onset Flag
Condition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedCondition not noted as arising during the episode of admitted patient care






Diagnosis 

Add Diagnosis



#Diagnosis TypeDiagnosis CodeActionsNo record found.




ui-buttonMedical Services


Code
Date
DrugPriceType
Qty
Price 
Total Price
Clinician:
Intervention Type
Sequence
daySupply
timeUnit
 
 




PackageTabletPackage




Other ServicePrincipalSecondaryOther Service



Add Service



#Service codeservice discerption 
                        App QtyPrice (SAR)Gross Amt (SAR)                                       PT + VATActionsNotes/ObsTotal-SAR0.000.00No record found.

                        
                           FP1NE 
                    Request Approval Request
                            Send Approval Request
                            Service codeAlert TypeQuan.NetClinicianNo record found.FPNERows Per Page1020304050FPNE#Response ResultAuthorization TypeTransaction DateNo records found.




PrimeFaces.cw(&quot;Poll&quot;,&quot;checkTHINKIntervalPoll&quot;,{id:&quot;SearchClaimForm:checkTHINKIntervalPoll&quot;,frequency:5,autoStart:true,fn:function(){PrimeFaces.ab({s:&quot;SearchClaimForm:checkTHINKIntervalPoll&quot;,f:&quot;SearchClaimForm&quot;,u:&quot;SearchClaimForm&quot;});}});Alert: This Claim Contains Errors , Execution Time : 108validate


1-
Severe  -
[K2_Effective_Date_of_Coverage]: Insurance Policy not valid on the Date of service. 






Visit Notes








Note Title










Note Type


Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesMissing Authorisation










Note Description














Add Note







Note TypeNote TitleNote DateDescriptionActionsNo records found.
Visit Attachments











Facility Type


Discharge reportGeneralLaboratoryOthersPrescriptionRadiologySurgeryDischarge report










Select File


Upload File


















Description














Create attachment
Cancel











File NameFacility TypeDescriptionPreviewActionsNo records found.


Rotate Left
Rotate Right
Scale +
Scale -
Update And Close
Download















ui-button


BP bpDiastolic:

BP Systolic:

Heart Rate



Height

Weight

Sao2



Spo2

RespiratoryRate

Temperature



Pb

Lmp

Illness duration



Illness Code
Check-upChronicCongenitalDental CleaningInfertilityMaternity ComplicationsOthersPregnancyPsychiatricRTATreatmentVaccinationWork RelatedCheck-up
UnitOfStay
DayMonthWeekYearDay
How



When

Where

Other









#ViewEditStatusDepartmentPatientStartEndpolicyHolderNamedenialCodeRef No.statusDescriptionadditionalInfoRequest DateResponse DateNo records found.





Move To AuditMove To Auditmoveclaim = function() {PrimeFaces.ab({s:&quot;actionsFormInvoice:updateInsertClaim1&quot;,f:&quot;actionsFormInvoice&quot;,u:&quot;actionsFormInvoice:updateInsertClaim1 growl actionsButtonsForm phWLForm:phWLTbl&quot;,onst:function(cfg){startAjaxLoader('MOVING TO CODING STAGE IN PROGRESS...');;},onco:function(xhr,status,args){endAjaxLoader();if(!args.validationFailed){PF('phWLTbl_widg').filter();PF('NewInvoiceWid').hide();PF('confirmationdlg').hide();}else{PF('confirmationdlg').show();} handleComplete(xhr,status,args);;},pa:arguments[0]});}Update Visitui-buttonHide 


LOINCTextFileFDIFinancialGroupingResultExcludeFromDRGERXRisk of MortalityProgramACHI_MAPPINGLOINCObservations



                        
                            


Observation Type
Observation Code
ObservationValue
Value Type




LOINCTextFileFDIFinancialGroupingResultExcludeFromDRGERXRisk of MortalityProgramACHI_MAPPINGLOINC
LOINCLOINC





   
                            
                            Add Observation
                            
                        file...Observation TypeObservation CodeObservationValueValue TypeActionsNo record found.CloseSave
Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesNotes








Note Type


Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesMissing Authorisation







Note Title










Add Note













Note Description








Note TypeNote TitleNote DateNote DescriptionActionsNo record found.CloseSave
id(&quot;InsuranceDetailsForm&quot;)/div[1]Export as ExcelDay Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryAllInsuranceCashAllNo servicesBilledPartially BilledRefundedNot Billed... SAAUDI(The Mediterranean &amp; Gulf Cooperative Insurance &amp; Reinsurance Company (MedGulf???????????Accumed Comany123132020-NACIG - TCSACIG- MEDNETAL ETIHADAl Sagr Cooperative  Insurance CompanyALBAHAALFA131ALJazira TakafulALJUMAIH COallamAllianz Saudi Fransi Insurance CompanyAlrajhi Takaful for Cooperative InsuranceAMANA INSURANCEARABIA InsuranceARABIAN SHEILDARAMCOAXA Cooperative Insurance CompanybandrbupaBupaBUPA Arabia for Cooperative InsuranceBURUJDate SeptEnvironmentETIHADfgfdgfvdgcfhgnvbGENERAL 2021GENERAL-2020-JgggGLOBEMEDGLOBEMED SAUDI ( GMS )GLOBEMED SAUDI ( GMS )GM-AMINAHGOSIGULF GENERAL - MEDNETGULF GENERAL - TCSGulf general Insurance CompanyGulf unionGulf Union Cooperative Insurance CompanytesthghgfdfdHKK Coinsurance company nameinsuranceTestJPJubil InsKHAMISKSAKSAKSA001Malath Cooperative Insurance &amp; Reinsurance CompanymaxmazMEDNETMOUNIRMS-NIZIHAmyNextcareRTRSAGR - MEDNETSALAMA INSURANCEsaudiSaudi Arabian cooperative Insurance Company (Saico)Saudi Enaya Cooperative InsuranceSaudi United  Cooperative Insurance  (WALA'A)sdfsSelf Pay - CashsolidaritysosoSPSRSYTawuniya cooperative  Insurance CompanyTawuniya cooperative  Insurance CompanyffgTCStest2020testAL RAJHItestAL SAGR - NEXTCAREtestMalathtestMalath CooperativetestMedGulftestRAJHIAAtestSAICOtestSAUDI ENAYAtestSGH STAFF - TAWUNIYAtestTAWUNIYAtestTawuniya cooperative Insurance CompanytestWALAAtestWALAA - NEXTCAREToday-27-07TOKIO MARINETOTAL CARE TOTAL CARE SAUDI (TCS)TRADE UNIONUAEUCAUNSPECIFIEDyamamahytPayer Patient Details


StatusPatientClassPolicy Holder NameDenial CodeReference NumberStatus DescriptionAdditional InfoCoveragesUpdate PatientHide 
...FemaleMaleAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D'IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE'S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE'S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWESelect One...DivorcedMarriedSeparatedSingleUnknownWidowedCreate Patient





MRN*National ID







Name*Middlename*Surname*Arabic First NameArabic Middle NameArabic Sure NameGender*...FemaleMale...Passport IDDOB / Age :*NationalityAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D'IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE'S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE'S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWEAFGHANISTANMarital StatusSelect One...DivorcedMarriedSeparatedSingleUnknownWidowedSelect One...HomeOfficeFaxMobilePersonEmailAdd Insurance Card



Member IDPayer IDPolicy/ClassStart DateEnd DateEditDeleteNo record found.


insurance  inquiry  request
Insert Patient
Cancel







cchi eligibility RequestPayers info


#InsurersRetry AllMessageCard NumberEffective DateExpiry DatePayerPlanNameMessageNo records found.



ClosePrimeFaces.cw(&quot;Poll&quot;,&quot;checkPayersThreads&quot;,{id:&quot;dhaInfoPanelForm:checkPayersThreads&quot;,frequency:5,autoStart:false,fn:function(){PrimeFaces.ab({s:&quot;dhaInfoPanelForm:checkPayersThreads&quot;,f:&quot;dhaInfoPanelForm&quot;,u:&quot;dhaInfoPanelForm:datalistAvailableEligibility&quot;,g:false,d:500});}});
...FemaleMaleAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D'IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE'S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE'S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWESelect One...DivorcedMarriedSeparatedSingleUnknownWidowedEdit Patient


MRN*National ID







Name*






Middlename*Surname*






Arabic First NameArabic Middle NameArabic Sure NameGender*


...FemaleMale...



Passport IDDOB / Age :*






Nationality


AFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D'IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE'S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE'S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWEAFGHANISTAN



Marital StatusSelect One...DivorcedMarriedSeparatedSingleUnknownWidowedSelect One...Home






OfficeFaxMobile






PersonEmail






Add Insurance CardMember IDPayer IDPolicy/ClassStart DateEnd DateEditDeleteNo record found.


Save
Cancel




Create Patient Card


GeneralInformationCopayment And DeductibleImages And other Information 


OP Copayment:

OP MaxPatientShare:



IP Copayment:

IP MaxPatientShare:




CopaymentCopayment CategoryCoPaymentAmountCeilingIP/OPEditDeleteNo record found.CreateDeductibleCopayment CategoryDeductableCeilingIP/OPEditDeleteNo record found.CreateCard Coverages*Covered ServicesNone Covered


DentalMaternityOptical












Foreground





















Background 


















Card Notes


Save
Cancel




ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate CoPayment


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitCoPayment Value*



%



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate Deductible


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitDeductible Value*



SAR



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate CoPayment


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitCoPayment Value*



%



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate Deductible


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitDeductible Value*



SAR



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
Edit Patient Card


GeneralInformationCopayment And Deductible Images And other Information


OP Copayment:

OP MaxPatientShare:



IP Copayment:

IP MaxPatientShare:



Prior Approval Limit:




CopaymentCopayment CategoryCoPaymentAmountCeilingApproval LimitIP/OPEditDeleteNo record found.CreateDeductibleCopayment CategoryDeductableCeilingApproval LimitIP/OPEditDeleteNo record found.CreateCard Coverages*Covered ServicesNone Covered


DentalMaternityOptical












Foreground





















Background


















Card Notes


Save
Cancel




Existing Patients


Similar patient records found based on values entered, to proceed please choose one of the following:Select from existing patient recordCreate new patient record


National IDNameMobileDOBMember IDStart DateExpiry DateNo records found.



ConfirmCancel
Insurance Details



Valid InsurancesExpired InsurancesNot Verified InsurancesCard NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.Patient Insurance NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.Patient Insurance NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.SaveAdd Insuranceinsurance  inquiry  requestCancel
CCHI Eligiblity Information


#PatientGenderNational IDpolicyNumberClassEffective DateExpiry DateCopayment And DeductibleIP MaxPatientShare:Payer NameCCHI infoNaphis InfoNo records found.HideUpdate Patient Alert






Patient exists in our system is differnet than one retreived from payer !





Existing Patient :
FGMNDCFHS EDGFRTHJU


Payer Patient: 










Update Patient
Cancel 












Payer Info            time out






Request to payer time out




                        


OK




                        
Insurance Co not found






Payer is not exist in data base please add it




                        
                           


OK




                           
detailsMissing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesDischarge reportGeneralLaboratoryOthersPrescriptionRadiologySurgeryPayer Patient Details




Payer Patient Details



Please check member eligibility as per the date of serviceNotes...Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular Surgery...Day Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital....Elective waiting list admission government free SchemeElective waiting list admission insurance coverage SchemeElective waiting list admission self-payment SchemeEmergency Admission by insurance companyEmergency Admission by referral from general government hospitalEmergency Admission by referral from government primary healthcare centerEmergency Admission by referral from private hospitalEmergency Admission by referral from private primary healthcare centerEmergency Admission from hospital EREmergency Admission from hospital outpatientEmergency Maternity Birth AdmissionImmediate AdmissionOthersPlanned Maternity Birth AdmissionUnknown...Discharge /Transfer to Other Health Care AccommodationDischarge /Transfer to a Residential Ageing ServiceDischarge/Transfer to a Psychiatric HospitalDischarge/transfer to an Acute HospitalHome/OtherStatistical Discharge-Type ChangeInitial client service eventSubsequent client service eventAcute CareBoarderComplex MaintenanceGeneral MaintenancePalliativeRehabilitation...Follow upNewReferralRefillWalk InAdmitted to this hospitalDead on arrivalDid not wait to be attended by a health care professionalDied in EDLeft at own risk after being attended by a health care professional, but before the non-admitted patient ED service episode was completedNon-admitted patient ED episode completed -referred to another hospital for admission;Non-admitted patient ED episode completed-departed without being admitted or referred to another hospitalRegistered, advised of another health care service, and left the EDwithout being attended by a health care professionalImmediate resuscitationNon-UrgentStandard ERUrgentVery UrgentadmittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfCondition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedPackageTabletOther ServicePrincipalSecondaryCheck-upChronicCongenitalDental CleaningInfertilityMaternity ComplicationsOthersPregnancyPsychiatricRTATreatmentVaccinationWork RelatedDayMonthWeekYearwarning






Validation error, Continue!




                        
                         
                        
                        ConfirmedCancel
</value>
      <webElementGuid>fb21cfb3-a0c8-4431-922e-336600847b4d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;ui-layout-container&quot;]</value>
      <webElementGuid>b657155d-be70-439a-a555-d599fc7fd564</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>67130e8f-a1bf-4cd1-aefe-5de3de5c8008</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;











OPENING VISIT IN PROGRESS...



Alert





















OK







    
        var ID, TYPE;

        function unloadPage() {
            console.log(&quot; , &quot;'&quot; , &quot;222&quot; , &quot;'&quot; , &quot;);
            unload();
        }
        function unload() {
            makeRequest( );
        }
        window.onbeforeunload = unloadPage;



        var facilityLicense;
        function loadRtlStyle(hrefpath) {
        }

        function removeRtlStyle() {
        }
        function applyEnglishLanguage() {
            deleteLang();
            setLang(&quot;lang&quot;, &quot;en&quot;, 365);
        }
        function applyArabicLanguage() {
            deleteLang();
            setLang(&quot;lang&quot;, &quot;ar&quot;, 365);
        }
        function setLang(cname, cvalue, exdays) {
            var d = new Date();
            d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
            var expires = &quot;expires=&quot; + d.toUTCString();
            document.cookie = cname + &quot;=&quot; + cvalue + &quot;;&quot; + expires + &quot;;path=/&quot;;
        }
        function getCurrentLangUpdate(cname) {
            var name = cname + &quot;=&quot;;
            var decodedCookie = decodeURIComponent(document.cookie);
            var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
            for (var i = 0; i != ca.length; i++) {
                var c = ca[i];
                while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;) {
                    c = c.substring(1);
                }
                if (c.indexOf(name) == 0) {
                    return c.substring(name.length, c.length);
                }
            }
            return &quot;&quot;;
        }
        function getCurrentLang(cname) {

            var name = cname + &quot;=&quot;;
            var decodedCookie = decodeURIComponent(document.cookie);
            var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
            for (var i = 0; i != ca.length; i++) {
                var c = ca[i];
                while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;) {
                    c = c.substring(1);
                }
                if (c.indexOf(name) == 0) {
                    return c.substring(name.length, c.length);
                }
            }
        }

        function deleteLang() {
            document.cookie = &quot;lang=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;&quot;;
        }

//           jQuery(function ($) {
//               if (getCurrentLang(&quot; , &quot;'&quot; , &quot;lang&quot; , &quot;'&quot; , &quot;) === &quot; , &quot;'&quot; , &quot;ar&quot; , &quot;'&quot; , &quot;) {
//                   var hrefCSS = &quot; , &quot;'&quot; , &quot;../resources/css/bootstrap-rtl.css&quot; , &quot;'&quot; , &quot;;
//                   loadRtlStyle(hrefCSS);
//               }
//            });
        var refreshArabicCss = false;
        function checkArabic() {
            if (getCurrentLang(&quot; , &quot;'&quot; , &quot;lang&quot; , &quot;'&quot; , &quot;) == &quot; , &quot;'&quot; , &quot;ar&quot; , &quot;'&quot; , &quot;) {

                let headelements = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;);
                for (var i = 0; i != headelements.length; i++) {
                    let headelement = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[i];
                    if (true) {
                        var x1 = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;);
                        if (x1[0].children[0].innerHTML != &quot; , &quot;'&quot; , &quot;الرئيسية&quot; , &quot;'&quot; , &quot;)
                        {
//      
                            setTimeout(refreshPage, 1000);
                        }

                    }
                }
            }
        }



        $(document).ready(function () {
        });
        function refreshPage() {
            location.reload();
        }
    

    
    
    
    


myCommand = function() {PrimeFaces.ab({s:&quot;j_idt37:j_idt38&quot;,f:&quot;j_idt37&quot;,pa:arguments[0]});}



HomePatient Journey Management Data Entry View VisitsCheck EligibilityOut Patient AuthorizationsIn Patient AuthorizationsOP BillingServicesOP ReceiptsER ReceiptsPharmacyPharmacy ListPharmacy POSIP BillingIP BillsIP Receipts Collections Daily collection Collection Daily Report Advanced Collection Report Deliver money to cashier administrator Settlement Receivable Advance CollectionRefund AdvanceRefund Receipts Definitions Patient Users Roles ContractUpload prices Manage Contracts Manage Price List Append Price Manage Payers Companies ProfilesClinicians Codes Mapping HL7 Log Back Office Management Batch Coding Auditing Submission Claim Submission Batch Submission Payment Management Upload Remittance Denial Management MIS Reports Visit Report









                    

                    
                        
                        
                    
administrator (King Salman Abdulaziz Hospital -Riyadh)HospitalKing Salman Abdulaziz Hospital -Riyadh                                                 
                                            Al-Jubail General Hospital - Jubail                                                 
                                            Ohod General Hospital -Medinah                                                 
                                            Al Yamamah Hospital - Riyadh                                                 
                                            Khamis Mushayt Maternity and children Hospital                                                 
                                            Al-Qrayat General Hospital                                                 
                                            Rabigh Hospital                                                 
                                            King Khalid Hospital-Tabouk                                                 
                                            Maternity &amp; Children Hospital – Tabuk                                                 
                                            Maternity and Children Hospital-Najran                                                 
                                            King Khaled Hospital  -Hail                                                 
                                            Maternity and Children Hospital -Hail                                                 
                                            Sabya General Hospital - Sabya                                                 
                                            Dubaa General Hospital-Tabouk                                                 
                                            King Fahad Hospital -Jeddah                                                 
                                            King Abdul Aziz Hospital and Oncology- Jeddah                                                 
                                            Maternity &amp; Children Hospital - Aljouf                                                 
                                            Prince Moteb bin Abdulaziz Hospital- Aljouf                                                 
                                            Tarif General Hospital - Tarif                                                 
                                            King Fahad Hospital - Al Baha                                                 
                                            King Khaled Hospital -Najran                                                 
                                            King Fahad Central Hospital - Hafouf                                                 
                                            King Khaled Hospital - Hafer Al Baten                                                 
                                            King Faisal Hospital-Taif                                                 
                                            Children Hospital in Taif                                                 
                                            King Abdullah Hospital - Beshah                                                 
                                            ALIMAN GENERAL HOSPITAL                                                 
                                            اللغة العربيةEnglishUser InfoLOGOUT







HomePatient Journey ManagementData Entryfocus = function() {PrimeFaces.ab({s:&quot;mainForm:j_idt169&quot;,f:&quot;mainForm&quot;,u:&quot;mainForm:facilityLicense&quot;,onco:function(xhr,status,args){checkChangeLicense();;},pa:arguments[0]});};$(function() {focus();});



$(function(){PrimeFaces.cw(&quot;Poll&quot;,&quot;checkModifiedChangedPoll&quot;,{id:&quot;phWLForm:checkModifiedChangedPoll&quot;,frequency:30000,autoStart:true,fn:function(){PrimeFaces.ab({s:&quot;phWLForm:checkModifiedChangedPoll&quot;,f:&quot;phWLForm&quot;,g:false});}});});onload = function() {PrimeFaces.ab({s:&quot;phWLForm:j_idt170&quot;,f:&quot;phWLForm&quot;,pa:arguments[0]});};$(function() {onload();});


Create New Visit Ammar

Search
Rerouted from Coding stage.








                                    Create New VisitExportui-button
                                    
                                    

                                       FP1NE Rows Per Page20304050607080
                                PatientFilter by PatientMRNFilter by MRNVisit NoFilter by Visit NoVisit DateAdmission Date 
                                      
                                    Visit DateDischarge Date 
                                      
                                    National IDFilter by National IDPhoneDepartmentFilter by DepartmentDoctorDay Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.FilterDepartmentAllergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryFilterDoctorFilter by DoctorPayment TypeAllInsuranceCashAllBill TypeAllNo servicesBilledPartially BilledRefundedNot BilledAllEligibility StatusPayer... SAAUDI(The Mediterranean &amp; Gulf Cooperative Insurance &amp; Reinsurance Company (MedGulf???????????Accumed Comany123132020-NACIG - TCSACIG- MEDNETAL ETIHADAl Sagr Cooperative  Insurance CompanyALBAHAALFA131ALJazira TakafulALJUMAIH COallamAllianz Saudi Fransi Insurance CompanyAlrajhi Takaful for Cooperative InsuranceAMANA INSURANCEARABIA InsuranceARABIAN SHEILDARAMCOAXA Cooperative Insurance CompanybandrbupaBupaBUPA Arabia for Cooperative InsuranceBURUJDate SeptEnvironmentETIHADfgfdgfvdgcfhgnvbGENERAL 2021GENERAL-2020-JgggGLOBEMEDGLOBEMED SAUDI ( GMS )GLOBEMED SAUDI ( GMS )GM-AMINAHGOSIGULF GENERAL - MEDNETGULF GENERAL - TCSGulf general Insurance CompanyGulf unionGulf Union Cooperative Insurance CompanytesthghgfdfdHKK Coinsurance company nameinsuranceTestJPJubil InsKHAMISKSAKSAKSA001Malath Cooperative Insurance &amp; Reinsurance CompanymaxmazMEDNETMOUNIRMS-NIZIHAmyNextcareRTRSAGR - MEDNETSALAMA INSURANCEsaudiSaudi Arabian cooperative Insurance Company (Saico)Saudi Enaya Cooperative InsuranceSaudi United  Cooperative Insurance  (WALA&quot; , &quot;'&quot; , &quot;A)sdfsSelf Pay - CashsolidaritysosoSPSRSYTawuniya cooperative  Insurance CompanyTawuniya cooperative  Insurance CompanyffgTCStest2020testAL RAJHItestAL SAGR - NEXTCAREtestMalathtestMalath CooperativetestMedGulftestRAJHIAAtestSAICOtestSAUDI ENAYAtestSGH STAFF - TAWUNIYAtestTAWUNIYAtestTawuniya cooperative Insurance CompanytestWALAAtestWALAA - NEXTCAREToday-27-07TOKIO MARINETOTAL CARE TOTAL CARE SAUDI (TCS)TRADE UNIONUAEUCAUNSPECIFIEDyamamahyt...DepartmentCreated ByLast Update ByDetailsfgMNDCFHS kjASDCFGG edgfrthju284596314APM-IP-000013525/11/202225/11/202225/11/202225/11/2022788491249594758126549521351 | No Bed + No Emergency RoomCardiologyDr. MUATH ALMUQHIMCashNo servicesPendingKSP001|Self Pay - Cash952135ui-button




Claim Audit#Login UserAudit DateClaim IDAction TypeAction IdentifierUpdated FieldsHIS Msg IdHIS Msg ActionHIS Msg TypeNo records found.FPNE    

YesNoCancel Prescription





Are you sure to cancel the prescription?


Please write down the reason for cancellation







                            Cancel AuthorizationClose       
                            
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfCondition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedPackageTabletOther ServicePrincipalSecondaryPBM Details


Accumed Patient InfoPatientNational IDDOB / AgeGenderMember IDMRNPayerId PackageCard Type:InsurancePrescription DateDoctor|


Diagnosis 


Diagnosis Date

Diagnosis Type
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfadmitting
Condition Onset Flag
Condition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedCondition not noted as arising during the episode of admitted patient care






Diagnosis 

Add Diagnosis



#Diagnosis TypeDiagnosis CodeActionsNo record found.



ui-buttonMedical Services


Code
Date
DrugPriceType
Qty
Price 
Total Price
Clinician:
Intervention Type
Sequence
daySupply
timeUnit
 
 




PackageTabletPackage




Other ServicePrincipalSecondaryOther Service



Add Service



#Service codeservice discerption 
                        App QtyPrice (SAR)Gross Amt (SAR)                                       PT + VATNet + VATActionsNotes/ObsTotal-SAR0.000.000.00No record found.

                        
                           FPNE 
                    Request Approval Request
                            Send Approval Request
                            Service codeAlert TypeQuan.NetClinicianNo record found.FPNERows Per Page1020304050FPNE#Response ResultAuthorization TypeTransaction DateNo records found.





                Send Request AuthorizationClose
                
Response Observations


(1 of 1) FPNENo records found.
Dispens Result





Priscreption failed to be dispensed,Sending submission file has been failed




Cancel Prescription





Are you sure to cancel the prescription?


Please write down the reason for cancellation







                Close       
                
Visit DetailsPatient &amp; Encounter InfoDiagnosis &amp; Consultation CodesCheck RulesVisit Notes / AttachmentsVital Signs DetailsEligibility History



ui-buttonsomeCommand = function() {PrimeFaces.ab({s:&quot;InvoiceForm:j_idt730&quot;,f:&quot;InvoiceForm&quot;,u:&quot;InvoiceForm:checkRequestsIntervalPoll InvoiceForm:editPatientButton InvoiceForm:patient InvoiceForm:cardInfo2 InvoiceForm:zoomPanel InvoiceForm:zoomPanel1 InvoiceForm:eligibilityRequests actionsFormInvoice InvoiceForm:copaymentBTN InvoiceForm:deductableBTN InvoiceForm:patientInsuranceGroup&quot;,pa:arguments[0]});}ui-buttonSearch Patient






New patient

                    
           
                    
Treatment Approval











Edit patient
LOG
Copayment
Deductible
Insurance Details






PatientfgMNDCFHS kjASDCFGG edgfrthjuNational ID7884912495NationalityAFGHANISTANDOB / Age15/08/1989, 33Y 3M 10DGenderMaleMarital:MarriedMember IDNO CARDMRN284596314Payment Type:Cash



Patient Audit#Login User Audit DateAction TypeAction IdentifierUpdated FieldsHIS Msg IdHIS Msg ActionHIS Msg TypeNo records found.FPNE


Rotate Left
Rotate Right
Scale +
Scale -
Download
















Rotate Left
Rotate Right
Scale +
Scale -
Download













FPNE#Response ResultEncounter TypeTransaction DateNo records found.#MeaningValueCeilingNo records found.ValueMeaningValueCeilingNo records found.Encounter


Visit Id*

Department 
...Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryCardiology
Start Date:*



End Date:

Encounter Type:*



...Day Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.Hospital Inpatient - patient occupies inpatient bed and stays overnight




Start Type:*
...Elective waiting list admission government free SchemeElective waiting list admission insurance coverage SchemeElective waiting list admission self-payment SchemeEmergency Admission by insurance companyEmergency Admission by referral from general government hospitalEmergency Admission by referral from government primary healthcare centerEmergency Admission by referral from private hospitalEmergency Admission by referral from private primary healthcare centerEmergency Admission from hospital EREmergency Admission from hospital outpatientEmergency Maternity Birth AdmissionImmediate AdmissionOthersPlanned Maternity Birth AdmissionUnknownOthers


End Type:
...Discharge /Transfer to Other Health Care AccommodationDischarge /Transfer to a Residential Ageing ServiceDischarge/Transfer to a Psychiatric HospitalDischarge/transfer to an Acute HospitalHome/OtherStatistical Discharge-Type Change...
Service Event Type:
Initial client service eventSubsequent client service eventInitial client service event
Care Type::
Acute CareBoarderComplex MaintenanceGeneral MaintenancePalliativeRehabilitationAcute Care


Visit Type
...Follow upNewReferralRefillWalk In...
Clinician:








Specialty  :
GENERAL PRACTICE GENERAL PRACTICE


Lenght of stay: 




Emergency Details


Dept Disposition:
Admitted to this hospitalDead on arrivalDid not wait to be attended by a health care professionalDied in EDLeft at own risk after being attended by a health care professional, but before the non-admitted patient ED service episode was completedNon-admitted patient ED episode completed -referred to another hospital for admission;Non-admitted patient ED episode completed-departed without being admitted or referred to another hospitalRegistered, advised of another health care service, and left the EDwithout being attended by a health care professionalAdmitted to this hospital
Service Date:



Waiting Time:

Arrival Code:



Triag Category
Immediate resuscitationNon-UrgentStandard ERUrgentVery UrgentImmediate resuscitation



Advanced


Speciality

Ward

Room



Bed

Cause Of Death

Past History



Chief Complaint

Physical Exam

Main Symptoms



Discharge Notes






    
        .apply {
            background-color: #fca752 !important;
            background-image: none !important;
            color: #000000 !important;
        }
    
    
    
    
    
    
    
        .cheap {
            background-color: #54a90a !important;

        }
    
    
        .pend {
            background-color: #3197d6 !important;

        }
    
    
        .rej {
            background-color: #e00b0b !important;

        }
    



Diagnosis 


Diagnosis Date

Diagnosis Type
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfadmitting
Condition Onset Flag
Condition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedCondition not noted as arising during the episode of admitted patient care






Diagnosis 

Add Diagnosis



#Diagnosis TypeDiagnosis CodeActionsNo record found.




ui-buttonMedical Services


Code
Date
DrugPriceType
Qty
Price 
Total Price
Clinician:
Intervention Type
Sequence
daySupply
timeUnit
 
 




PackageTabletPackage




Other ServicePrincipalSecondaryOther Service



Add Service



#Service codeservice discerption 
                        App QtyPrice (SAR)Gross Amt (SAR)                                       PT + VATActionsNotes/ObsTotal-SAR0.000.00No record found.

                        
                           FP1NE 
                    Request Approval Request
                            Send Approval Request
                            Service codeAlert TypeQuan.NetClinicianNo record found.FPNERows Per Page1020304050FPNE#Response ResultAuthorization TypeTransaction DateNo records found.




PrimeFaces.cw(&quot;Poll&quot;,&quot;checkTHINKIntervalPoll&quot;,{id:&quot;SearchClaimForm:checkTHINKIntervalPoll&quot;,frequency:5,autoStart:true,fn:function(){PrimeFaces.ab({s:&quot;SearchClaimForm:checkTHINKIntervalPoll&quot;,f:&quot;SearchClaimForm&quot;,u:&quot;SearchClaimForm&quot;});}});Alert: This Claim Contains Errors , Execution Time : 108validate


1-
Severe  -
[K2_Effective_Date_of_Coverage]: Insurance Policy not valid on the Date of service. 






Visit Notes








Note Title










Note Type


Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesMissing Authorisation










Note Description














Add Note







Note TypeNote TitleNote DateDescriptionActionsNo records found.
Visit Attachments











Facility Type


Discharge reportGeneralLaboratoryOthersPrescriptionRadiologySurgeryDischarge report










Select File


Upload File


















Description














Create attachment
Cancel











File NameFacility TypeDescriptionPreviewActionsNo records found.


Rotate Left
Rotate Right
Scale +
Scale -
Update And Close
Download















ui-button


BP bpDiastolic:

BP Systolic:

Heart Rate



Height

Weight

Sao2



Spo2

RespiratoryRate

Temperature



Pb

Lmp

Illness duration



Illness Code
Check-upChronicCongenitalDental CleaningInfertilityMaternity ComplicationsOthersPregnancyPsychiatricRTATreatmentVaccinationWork RelatedCheck-up
UnitOfStay
DayMonthWeekYearDay
How



When

Where

Other









#ViewEditStatusDepartmentPatientStartEndpolicyHolderNamedenialCodeRef No.statusDescriptionadditionalInfoRequest DateResponse DateNo records found.





Move To AuditMove To Auditmoveclaim = function() {PrimeFaces.ab({s:&quot;actionsFormInvoice:updateInsertClaim1&quot;,f:&quot;actionsFormInvoice&quot;,u:&quot;actionsFormInvoice:updateInsertClaim1 growl actionsButtonsForm phWLForm:phWLTbl&quot;,onst:function(cfg){startAjaxLoader(&quot; , &quot;'&quot; , &quot;MOVING TO CODING STAGE IN PROGRESS...&quot; , &quot;'&quot; , &quot;);;},onco:function(xhr,status,args){endAjaxLoader();if(!args.validationFailed){PF(&quot; , &quot;'&quot; , &quot;phWLTbl_widg&quot; , &quot;'&quot; , &quot;).filter();PF(&quot; , &quot;'&quot; , &quot;NewInvoiceWid&quot; , &quot;'&quot; , &quot;).hide();PF(&quot; , &quot;'&quot; , &quot;confirmationdlg&quot; , &quot;'&quot; , &quot;).hide();}else{PF(&quot; , &quot;'&quot; , &quot;confirmationdlg&quot; , &quot;'&quot; , &quot;).show();} handleComplete(xhr,status,args);;},pa:arguments[0]});}Update Visitui-buttonHide 


LOINCTextFileFDIFinancialGroupingResultExcludeFromDRGERXRisk of MortalityProgramACHI_MAPPINGLOINCObservations



                        
                            


Observation Type
Observation Code
ObservationValue
Value Type




LOINCTextFileFDIFinancialGroupingResultExcludeFromDRGERXRisk of MortalityProgramACHI_MAPPINGLOINC
LOINCLOINC





   
                            
                            Add Observation
                            
                        file...Observation TypeObservation CodeObservationValueValue TypeActionsNo record found.CloseSave
Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesNotes








Note Type


Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesMissing Authorisation







Note Title










Add Note













Note Description








Note TypeNote TitleNote DateNote DescriptionActionsNo record found.CloseSave
id(&quot;InsuranceDetailsForm&quot;)/div[1]Export as ExcelDay Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryAllInsuranceCashAllNo servicesBilledPartially BilledRefundedNot Billed... SAAUDI(The Mediterranean &amp; Gulf Cooperative Insurance &amp; Reinsurance Company (MedGulf???????????Accumed Comany123132020-NACIG - TCSACIG- MEDNETAL ETIHADAl Sagr Cooperative  Insurance CompanyALBAHAALFA131ALJazira TakafulALJUMAIH COallamAllianz Saudi Fransi Insurance CompanyAlrajhi Takaful for Cooperative InsuranceAMANA INSURANCEARABIA InsuranceARABIAN SHEILDARAMCOAXA Cooperative Insurance CompanybandrbupaBupaBUPA Arabia for Cooperative InsuranceBURUJDate SeptEnvironmentETIHADfgfdgfvdgcfhgnvbGENERAL 2021GENERAL-2020-JgggGLOBEMEDGLOBEMED SAUDI ( GMS )GLOBEMED SAUDI ( GMS )GM-AMINAHGOSIGULF GENERAL - MEDNETGULF GENERAL - TCSGulf general Insurance CompanyGulf unionGulf Union Cooperative Insurance CompanytesthghgfdfdHKK Coinsurance company nameinsuranceTestJPJubil InsKHAMISKSAKSAKSA001Malath Cooperative Insurance &amp; Reinsurance CompanymaxmazMEDNETMOUNIRMS-NIZIHAmyNextcareRTRSAGR - MEDNETSALAMA INSURANCEsaudiSaudi Arabian cooperative Insurance Company (Saico)Saudi Enaya Cooperative InsuranceSaudi United  Cooperative Insurance  (WALA&quot; , &quot;'&quot; , &quot;A)sdfsSelf Pay - CashsolidaritysosoSPSRSYTawuniya cooperative  Insurance CompanyTawuniya cooperative  Insurance CompanyffgTCStest2020testAL RAJHItestAL SAGR - NEXTCAREtestMalathtestMalath CooperativetestMedGulftestRAJHIAAtestSAICOtestSAUDI ENAYAtestSGH STAFF - TAWUNIYAtestTAWUNIYAtestTawuniya cooperative Insurance CompanytestWALAAtestWALAA - NEXTCAREToday-27-07TOKIO MARINETOTAL CARE TOTAL CARE SAUDI (TCS)TRADE UNIONUAEUCAUNSPECIFIEDyamamahytPayer Patient Details


StatusPatientClassPolicy Holder NameDenial CodeReference NumberStatus DescriptionAdditional InfoCoveragesUpdate PatientHide 
...FemaleMaleAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWESelect One...DivorcedMarriedSeparatedSingleUnknownWidowedCreate Patient





MRN*National ID







Name*Middlename*Surname*Arabic First NameArabic Middle NameArabic Sure NameGender*...FemaleMale...Passport IDDOB / Age :*NationalityAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWEAFGHANISTANMarital StatusSelect One...DivorcedMarriedSeparatedSingleUnknownWidowedSelect One...HomeOfficeFaxMobilePersonEmailAdd Insurance Card



Member IDPayer IDPolicy/ClassStart DateEnd DateEditDeleteNo record found.


insurance  inquiry  request
Insert Patient
Cancel







cchi eligibility RequestPayers info


#InsurersRetry AllMessageCard NumberEffective DateExpiry DatePayerPlanNameMessageNo records found.



ClosePrimeFaces.cw(&quot;Poll&quot;,&quot;checkPayersThreads&quot;,{id:&quot;dhaInfoPanelForm:checkPayersThreads&quot;,frequency:5,autoStart:false,fn:function(){PrimeFaces.ab({s:&quot;dhaInfoPanelForm:checkPayersThreads&quot;,f:&quot;dhaInfoPanelForm&quot;,u:&quot;dhaInfoPanelForm:datalistAvailableEligibility&quot;,g:false,d:500});}});
...FemaleMaleAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWESelect One...DivorcedMarriedSeparatedSingleUnknownWidowedEdit Patient


MRN*National ID







Name*






Middlename*Surname*






Arabic First NameArabic Middle NameArabic Sure NameGender*


...FemaleMale...



Passport IDDOB / Age :*






Nationality


AFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWEAFGHANISTAN



Marital StatusSelect One...DivorcedMarriedSeparatedSingleUnknownWidowedSelect One...Home






OfficeFaxMobile






PersonEmail






Add Insurance CardMember IDPayer IDPolicy/ClassStart DateEnd DateEditDeleteNo record found.


Save
Cancel




Create Patient Card


GeneralInformationCopayment And DeductibleImages And other Information 


OP Copayment:

OP MaxPatientShare:



IP Copayment:

IP MaxPatientShare:




CopaymentCopayment CategoryCoPaymentAmountCeilingIP/OPEditDeleteNo record found.CreateDeductibleCopayment CategoryDeductableCeilingIP/OPEditDeleteNo record found.CreateCard Coverages*Covered ServicesNone Covered


DentalMaternityOptical












Foreground





















Background 


















Card Notes


Save
Cancel




ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate CoPayment


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitCoPayment Value*



%



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate Deductible


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitDeductible Value*



SAR



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate CoPayment


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitCoPayment Value*



%



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate Deductible


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitDeductible Value*



SAR



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
Edit Patient Card


GeneralInformationCopayment And Deductible Images And other Information


OP Copayment:

OP MaxPatientShare:



IP Copayment:

IP MaxPatientShare:



Prior Approval Limit:




CopaymentCopayment CategoryCoPaymentAmountCeilingApproval LimitIP/OPEditDeleteNo record found.CreateDeductibleCopayment CategoryDeductableCeilingApproval LimitIP/OPEditDeleteNo record found.CreateCard Coverages*Covered ServicesNone Covered


DentalMaternityOptical












Foreground





















Background


















Card Notes


Save
Cancel




Existing Patients


Similar patient records found based on values entered, to proceed please choose one of the following:Select from existing patient recordCreate new patient record


National IDNameMobileDOBMember IDStart DateExpiry DateNo records found.



ConfirmCancel
Insurance Details



Valid InsurancesExpired InsurancesNot Verified InsurancesCard NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.Patient Insurance NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.Patient Insurance NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.SaveAdd Insuranceinsurance  inquiry  requestCancel
CCHI Eligiblity Information


#PatientGenderNational IDpolicyNumberClassEffective DateExpiry DateCopayment And DeductibleIP MaxPatientShare:Payer NameCCHI infoNaphis InfoNo records found.HideUpdate Patient Alert






Patient exists in our system is differnet than one retreived from payer !





Existing Patient :
FGMNDCFHS EDGFRTHJU


Payer Patient: 










Update Patient
Cancel 












Payer Info            time out






Request to payer time out




                        


OK




                        
Insurance Co not found






Payer is not exist in data base please add it




                        
                           


OK




                           
detailsMissing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesDischarge reportGeneralLaboratoryOthersPrescriptionRadiologySurgeryPayer Patient Details




Payer Patient Details



Please check member eligibility as per the date of serviceNotes...Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular Surgery...Day Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital....Elective waiting list admission government free SchemeElective waiting list admission insurance coverage SchemeElective waiting list admission self-payment SchemeEmergency Admission by insurance companyEmergency Admission by referral from general government hospitalEmergency Admission by referral from government primary healthcare centerEmergency Admission by referral from private hospitalEmergency Admission by referral from private primary healthcare centerEmergency Admission from hospital EREmergency Admission from hospital outpatientEmergency Maternity Birth AdmissionImmediate AdmissionOthersPlanned Maternity Birth AdmissionUnknown...Discharge /Transfer to Other Health Care AccommodationDischarge /Transfer to a Residential Ageing ServiceDischarge/Transfer to a Psychiatric HospitalDischarge/transfer to an Acute HospitalHome/OtherStatistical Discharge-Type ChangeInitial client service eventSubsequent client service eventAcute CareBoarderComplex MaintenanceGeneral MaintenancePalliativeRehabilitation...Follow upNewReferralRefillWalk InAdmitted to this hospitalDead on arrivalDid not wait to be attended by a health care professionalDied in EDLeft at own risk after being attended by a health care professional, but before the non-admitted patient ED service episode was completedNon-admitted patient ED episode completed -referred to another hospital for admission;Non-admitted patient ED episode completed-departed without being admitted or referred to another hospitalRegistered, advised of another health care service, and left the EDwithout being attended by a health care professionalImmediate resuscitationNon-UrgentStandard ERUrgentVery UrgentadmittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfCondition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedPackageTabletOther ServicePrincipalSecondaryCheck-upChronicCongenitalDental CleaningInfertilityMaternity ComplicationsOthersPregnancyPsychiatricRTATreatmentVaccinationWork RelatedDayMonthWeekYearwarning






Validation error, Continue!




                        
                         
                        
                        ConfirmedCancel
&quot;) or . = concat(&quot;











OPENING VISIT IN PROGRESS...



Alert





















OK







    
        var ID, TYPE;

        function unloadPage() {
            console.log(&quot; , &quot;'&quot; , &quot;222&quot; , &quot;'&quot; , &quot;);
            unload();
        }
        function unload() {
            makeRequest( );
        }
        window.onbeforeunload = unloadPage;



        var facilityLicense;
        function loadRtlStyle(hrefpath) {
        }

        function removeRtlStyle() {
        }
        function applyEnglishLanguage() {
            deleteLang();
            setLang(&quot;lang&quot;, &quot;en&quot;, 365);
        }
        function applyArabicLanguage() {
            deleteLang();
            setLang(&quot;lang&quot;, &quot;ar&quot;, 365);
        }
        function setLang(cname, cvalue, exdays) {
            var d = new Date();
            d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
            var expires = &quot;expires=&quot; + d.toUTCString();
            document.cookie = cname + &quot;=&quot; + cvalue + &quot;;&quot; + expires + &quot;;path=/&quot;;
        }
        function getCurrentLangUpdate(cname) {
            var name = cname + &quot;=&quot;;
            var decodedCookie = decodeURIComponent(document.cookie);
            var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
            for (var i = 0; i != ca.length; i++) {
                var c = ca[i];
                while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;) {
                    c = c.substring(1);
                }
                if (c.indexOf(name) == 0) {
                    return c.substring(name.length, c.length);
                }
            }
            return &quot;&quot;;
        }
        function getCurrentLang(cname) {

            var name = cname + &quot;=&quot;;
            var decodedCookie = decodeURIComponent(document.cookie);
            var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
            for (var i = 0; i != ca.length; i++) {
                var c = ca[i];
                while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;) {
                    c = c.substring(1);
                }
                if (c.indexOf(name) == 0) {
                    return c.substring(name.length, c.length);
                }
            }
        }

        function deleteLang() {
            document.cookie = &quot;lang=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;&quot;;
        }

//           jQuery(function ($) {
//               if (getCurrentLang(&quot; , &quot;'&quot; , &quot;lang&quot; , &quot;'&quot; , &quot;) === &quot; , &quot;'&quot; , &quot;ar&quot; , &quot;'&quot; , &quot;) {
//                   var hrefCSS = &quot; , &quot;'&quot; , &quot;../resources/css/bootstrap-rtl.css&quot; , &quot;'&quot; , &quot;;
//                   loadRtlStyle(hrefCSS);
//               }
//            });
        var refreshArabicCss = false;
        function checkArabic() {
            if (getCurrentLang(&quot; , &quot;'&quot; , &quot;lang&quot; , &quot;'&quot; , &quot;) == &quot; , &quot;'&quot; , &quot;ar&quot; , &quot;'&quot; , &quot;) {

                let headelements = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;);
                for (var i = 0; i != headelements.length; i++) {
                    let headelement = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[i];
                    if (true) {
                        var x1 = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;);
                        if (x1[0].children[0].innerHTML != &quot; , &quot;'&quot; , &quot;الرئيسية&quot; , &quot;'&quot; , &quot;)
                        {
//      
                            setTimeout(refreshPage, 1000);
                        }

                    }
                }
            }
        }



        $(document).ready(function () {
        });
        function refreshPage() {
            location.reload();
        }
    

    
    
    
    


myCommand = function() {PrimeFaces.ab({s:&quot;j_idt37:j_idt38&quot;,f:&quot;j_idt37&quot;,pa:arguments[0]});}



HomePatient Journey Management Data Entry View VisitsCheck EligibilityOut Patient AuthorizationsIn Patient AuthorizationsOP BillingServicesOP ReceiptsER ReceiptsPharmacyPharmacy ListPharmacy POSIP BillingIP BillsIP Receipts Collections Daily collection Collection Daily Report Advanced Collection Report Deliver money to cashier administrator Settlement Receivable Advance CollectionRefund AdvanceRefund Receipts Definitions Patient Users Roles ContractUpload prices Manage Contracts Manage Price List Append Price Manage Payers Companies ProfilesClinicians Codes Mapping HL7 Log Back Office Management Batch Coding Auditing Submission Claim Submission Batch Submission Payment Management Upload Remittance Denial Management MIS Reports Visit Report









                    

                    
                        
                        
                    
administrator (King Salman Abdulaziz Hospital -Riyadh)HospitalKing Salman Abdulaziz Hospital -Riyadh                                                 
                                            Al-Jubail General Hospital - Jubail                                                 
                                            Ohod General Hospital -Medinah                                                 
                                            Al Yamamah Hospital - Riyadh                                                 
                                            Khamis Mushayt Maternity and children Hospital                                                 
                                            Al-Qrayat General Hospital                                                 
                                            Rabigh Hospital                                                 
                                            King Khalid Hospital-Tabouk                                                 
                                            Maternity &amp; Children Hospital – Tabuk                                                 
                                            Maternity and Children Hospital-Najran                                                 
                                            King Khaled Hospital  -Hail                                                 
                                            Maternity and Children Hospital -Hail                                                 
                                            Sabya General Hospital - Sabya                                                 
                                            Dubaa General Hospital-Tabouk                                                 
                                            King Fahad Hospital -Jeddah                                                 
                                            King Abdul Aziz Hospital and Oncology- Jeddah                                                 
                                            Maternity &amp; Children Hospital - Aljouf                                                 
                                            Prince Moteb bin Abdulaziz Hospital- Aljouf                                                 
                                            Tarif General Hospital - Tarif                                                 
                                            King Fahad Hospital - Al Baha                                                 
                                            King Khaled Hospital -Najran                                                 
                                            King Fahad Central Hospital - Hafouf                                                 
                                            King Khaled Hospital - Hafer Al Baten                                                 
                                            King Faisal Hospital-Taif                                                 
                                            Children Hospital in Taif                                                 
                                            King Abdullah Hospital - Beshah                                                 
                                            ALIMAN GENERAL HOSPITAL                                                 
                                            اللغة العربيةEnglishUser InfoLOGOUT







HomePatient Journey ManagementData Entryfocus = function() {PrimeFaces.ab({s:&quot;mainForm:j_idt169&quot;,f:&quot;mainForm&quot;,u:&quot;mainForm:facilityLicense&quot;,onco:function(xhr,status,args){checkChangeLicense();;},pa:arguments[0]});};$(function() {focus();});



$(function(){PrimeFaces.cw(&quot;Poll&quot;,&quot;checkModifiedChangedPoll&quot;,{id:&quot;phWLForm:checkModifiedChangedPoll&quot;,frequency:30000,autoStart:true,fn:function(){PrimeFaces.ab({s:&quot;phWLForm:checkModifiedChangedPoll&quot;,f:&quot;phWLForm&quot;,g:false});}});});onload = function() {PrimeFaces.ab({s:&quot;phWLForm:j_idt170&quot;,f:&quot;phWLForm&quot;,pa:arguments[0]});};$(function() {onload();});


Create New Visit Ammar

Search
Rerouted from Coding stage.








                                    Create New VisitExportui-button
                                    
                                    

                                       FP1NE Rows Per Page20304050607080
                                PatientFilter by PatientMRNFilter by MRNVisit NoFilter by Visit NoVisit DateAdmission Date 
                                      
                                    Visit DateDischarge Date 
                                      
                                    National IDFilter by National IDPhoneDepartmentFilter by DepartmentDoctorDay Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.FilterDepartmentAllergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryFilterDoctorFilter by DoctorPayment TypeAllInsuranceCashAllBill TypeAllNo servicesBilledPartially BilledRefundedNot BilledAllEligibility StatusPayer... SAAUDI(The Mediterranean &amp; Gulf Cooperative Insurance &amp; Reinsurance Company (MedGulf???????????Accumed Comany123132020-NACIG - TCSACIG- MEDNETAL ETIHADAl Sagr Cooperative  Insurance CompanyALBAHAALFA131ALJazira TakafulALJUMAIH COallamAllianz Saudi Fransi Insurance CompanyAlrajhi Takaful for Cooperative InsuranceAMANA INSURANCEARABIA InsuranceARABIAN SHEILDARAMCOAXA Cooperative Insurance CompanybandrbupaBupaBUPA Arabia for Cooperative InsuranceBURUJDate SeptEnvironmentETIHADfgfdgfvdgcfhgnvbGENERAL 2021GENERAL-2020-JgggGLOBEMEDGLOBEMED SAUDI ( GMS )GLOBEMED SAUDI ( GMS )GM-AMINAHGOSIGULF GENERAL - MEDNETGULF GENERAL - TCSGulf general Insurance CompanyGulf unionGulf Union Cooperative Insurance CompanytesthghgfdfdHKK Coinsurance company nameinsuranceTestJPJubil InsKHAMISKSAKSAKSA001Malath Cooperative Insurance &amp; Reinsurance CompanymaxmazMEDNETMOUNIRMS-NIZIHAmyNextcareRTRSAGR - MEDNETSALAMA INSURANCEsaudiSaudi Arabian cooperative Insurance Company (Saico)Saudi Enaya Cooperative InsuranceSaudi United  Cooperative Insurance  (WALA&quot; , &quot;'&quot; , &quot;A)sdfsSelf Pay - CashsolidaritysosoSPSRSYTawuniya cooperative  Insurance CompanyTawuniya cooperative  Insurance CompanyffgTCStest2020testAL RAJHItestAL SAGR - NEXTCAREtestMalathtestMalath CooperativetestMedGulftestRAJHIAAtestSAICOtestSAUDI ENAYAtestSGH STAFF - TAWUNIYAtestTAWUNIYAtestTawuniya cooperative Insurance CompanytestWALAAtestWALAA - NEXTCAREToday-27-07TOKIO MARINETOTAL CARE TOTAL CARE SAUDI (TCS)TRADE UNIONUAEUCAUNSPECIFIEDyamamahyt...DepartmentCreated ByLast Update ByDetailsfgMNDCFHS kjASDCFGG edgfrthju284596314APM-IP-000013525/11/202225/11/202225/11/202225/11/2022788491249594758126549521351 | No Bed + No Emergency RoomCardiologyDr. MUATH ALMUQHIMCashNo servicesPendingKSP001|Self Pay - Cash952135ui-button




Claim Audit#Login UserAudit DateClaim IDAction TypeAction IdentifierUpdated FieldsHIS Msg IdHIS Msg ActionHIS Msg TypeNo records found.FPNE    

YesNoCancel Prescription





Are you sure to cancel the prescription?


Please write down the reason for cancellation







                            Cancel AuthorizationClose       
                            
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfCondition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedPackageTabletOther ServicePrincipalSecondaryPBM Details


Accumed Patient InfoPatientNational IDDOB / AgeGenderMember IDMRNPayerId PackageCard Type:InsurancePrescription DateDoctor|


Diagnosis 


Diagnosis Date

Diagnosis Type
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfadmitting
Condition Onset Flag
Condition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedCondition not noted as arising during the episode of admitted patient care






Diagnosis 

Add Diagnosis



#Diagnosis TypeDiagnosis CodeActionsNo record found.



ui-buttonMedical Services


Code
Date
DrugPriceType
Qty
Price 
Total Price
Clinician:
Intervention Type
Sequence
daySupply
timeUnit
 
 




PackageTabletPackage




Other ServicePrincipalSecondaryOther Service



Add Service



#Service codeservice discerption 
                        App QtyPrice (SAR)Gross Amt (SAR)                                       PT + VATNet + VATActionsNotes/ObsTotal-SAR0.000.000.00No record found.

                        
                           FPNE 
                    Request Approval Request
                            Send Approval Request
                            Service codeAlert TypeQuan.NetClinicianNo record found.FPNERows Per Page1020304050FPNE#Response ResultAuthorization TypeTransaction DateNo records found.





                Send Request AuthorizationClose
                
Response Observations


(1 of 1) FPNENo records found.
Dispens Result





Priscreption failed to be dispensed,Sending submission file has been failed




Cancel Prescription





Are you sure to cancel the prescription?


Please write down the reason for cancellation







                Close       
                
Visit DetailsPatient &amp; Encounter InfoDiagnosis &amp; Consultation CodesCheck RulesVisit Notes / AttachmentsVital Signs DetailsEligibility History



ui-buttonsomeCommand = function() {PrimeFaces.ab({s:&quot;InvoiceForm:j_idt730&quot;,f:&quot;InvoiceForm&quot;,u:&quot;InvoiceForm:checkRequestsIntervalPoll InvoiceForm:editPatientButton InvoiceForm:patient InvoiceForm:cardInfo2 InvoiceForm:zoomPanel InvoiceForm:zoomPanel1 InvoiceForm:eligibilityRequests actionsFormInvoice InvoiceForm:copaymentBTN InvoiceForm:deductableBTN InvoiceForm:patientInsuranceGroup&quot;,pa:arguments[0]});}ui-buttonSearch Patient






New patient

                    
           
                    
Treatment Approval











Edit patient
LOG
Copayment
Deductible
Insurance Details






PatientfgMNDCFHS kjASDCFGG edgfrthjuNational ID7884912495NationalityAFGHANISTANDOB / Age15/08/1989, 33Y 3M 10DGenderMaleMarital:MarriedMember IDNO CARDMRN284596314Payment Type:Cash



Patient Audit#Login User Audit DateAction TypeAction IdentifierUpdated FieldsHIS Msg IdHIS Msg ActionHIS Msg TypeNo records found.FPNE


Rotate Left
Rotate Right
Scale +
Scale -
Download
















Rotate Left
Rotate Right
Scale +
Scale -
Download













FPNE#Response ResultEncounter TypeTransaction DateNo records found.#MeaningValueCeilingNo records found.ValueMeaningValueCeilingNo records found.Encounter


Visit Id*

Department 
...Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryCardiology
Start Date:*



End Date:

Encounter Type:*



...Day Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.Hospital Inpatient - patient occupies inpatient bed and stays overnight




Start Type:*
...Elective waiting list admission government free SchemeElective waiting list admission insurance coverage SchemeElective waiting list admission self-payment SchemeEmergency Admission by insurance companyEmergency Admission by referral from general government hospitalEmergency Admission by referral from government primary healthcare centerEmergency Admission by referral from private hospitalEmergency Admission by referral from private primary healthcare centerEmergency Admission from hospital EREmergency Admission from hospital outpatientEmergency Maternity Birth AdmissionImmediate AdmissionOthersPlanned Maternity Birth AdmissionUnknownOthers


End Type:
...Discharge /Transfer to Other Health Care AccommodationDischarge /Transfer to a Residential Ageing ServiceDischarge/Transfer to a Psychiatric HospitalDischarge/transfer to an Acute HospitalHome/OtherStatistical Discharge-Type Change...
Service Event Type:
Initial client service eventSubsequent client service eventInitial client service event
Care Type::
Acute CareBoarderComplex MaintenanceGeneral MaintenancePalliativeRehabilitationAcute Care


Visit Type
...Follow upNewReferralRefillWalk In...
Clinician:








Specialty  :
GENERAL PRACTICE GENERAL PRACTICE


Lenght of stay: 




Emergency Details


Dept Disposition:
Admitted to this hospitalDead on arrivalDid not wait to be attended by a health care professionalDied in EDLeft at own risk after being attended by a health care professional, but before the non-admitted patient ED service episode was completedNon-admitted patient ED episode completed -referred to another hospital for admission;Non-admitted patient ED episode completed-departed without being admitted or referred to another hospitalRegistered, advised of another health care service, and left the EDwithout being attended by a health care professionalAdmitted to this hospital
Service Date:



Waiting Time:

Arrival Code:



Triag Category
Immediate resuscitationNon-UrgentStandard ERUrgentVery UrgentImmediate resuscitation



Advanced


Speciality

Ward

Room



Bed

Cause Of Death

Past History



Chief Complaint

Physical Exam

Main Symptoms



Discharge Notes






    
        .apply {
            background-color: #fca752 !important;
            background-image: none !important;
            color: #000000 !important;
        }
    
    
    
    
    
    
    
        .cheap {
            background-color: #54a90a !important;

        }
    
    
        .pend {
            background-color: #3197d6 !important;

        }
    
    
        .rej {
            background-color: #e00b0b !important;

        }
    



Diagnosis 


Diagnosis Date

Diagnosis Type
admittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfadmitting
Condition Onset Flag
Condition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedCondition not noted as arising during the episode of admitted patient care






Diagnosis 

Add Diagnosis



#Diagnosis TypeDiagnosis CodeActionsNo record found.




ui-buttonMedical Services


Code
Date
DrugPriceType
Qty
Price 
Total Price
Clinician:
Intervention Type
Sequence
daySupply
timeUnit
 
 




PackageTabletPackage




Other ServicePrincipalSecondaryOther Service



Add Service



#Service codeservice discerption 
                        App QtyPrice (SAR)Gross Amt (SAR)                                       PT + VATActionsNotes/ObsTotal-SAR0.000.00No record found.

                        
                           FP1NE 
                    Request Approval Request
                            Send Approval Request
                            Service codeAlert TypeQuan.NetClinicianNo record found.FPNERows Per Page1020304050FPNE#Response ResultAuthorization TypeTransaction DateNo records found.




PrimeFaces.cw(&quot;Poll&quot;,&quot;checkTHINKIntervalPoll&quot;,{id:&quot;SearchClaimForm:checkTHINKIntervalPoll&quot;,frequency:5,autoStart:true,fn:function(){PrimeFaces.ab({s:&quot;SearchClaimForm:checkTHINKIntervalPoll&quot;,f:&quot;SearchClaimForm&quot;,u:&quot;SearchClaimForm&quot;});}});Alert: This Claim Contains Errors , Execution Time : 108validate


1-
Severe  -
[K2_Effective_Date_of_Coverage]: Insurance Policy not valid on the Date of service. 






Visit Notes








Note Title










Note Type


Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesMissing Authorisation










Note Description














Add Note







Note TypeNote TitleNote DateDescriptionActionsNo records found.
Visit Attachments











Facility Type


Discharge reportGeneralLaboratoryOthersPrescriptionRadiologySurgeryDischarge report










Select File


Upload File


















Description














Create attachment
Cancel











File NameFacility TypeDescriptionPreviewActionsNo records found.


Rotate Left
Rotate Right
Scale +
Scale -
Update And Close
Download















ui-button


BP bpDiastolic:

BP Systolic:

Heart Rate



Height

Weight

Sao2



Spo2

RespiratoryRate

Temperature



Pb

Lmp

Illness duration



Illness Code
Check-upChronicCongenitalDental CleaningInfertilityMaternity ComplicationsOthersPregnancyPsychiatricRTATreatmentVaccinationWork RelatedCheck-up
UnitOfStay
DayMonthWeekYearDay
How



When

Where

Other









#ViewEditStatusDepartmentPatientStartEndpolicyHolderNamedenialCodeRef No.statusDescriptionadditionalInfoRequest DateResponse DateNo records found.





Move To AuditMove To Auditmoveclaim = function() {PrimeFaces.ab({s:&quot;actionsFormInvoice:updateInsertClaim1&quot;,f:&quot;actionsFormInvoice&quot;,u:&quot;actionsFormInvoice:updateInsertClaim1 growl actionsButtonsForm phWLForm:phWLTbl&quot;,onst:function(cfg){startAjaxLoader(&quot; , &quot;'&quot; , &quot;MOVING TO CODING STAGE IN PROGRESS...&quot; , &quot;'&quot; , &quot;);;},onco:function(xhr,status,args){endAjaxLoader();if(!args.validationFailed){PF(&quot; , &quot;'&quot; , &quot;phWLTbl_widg&quot; , &quot;'&quot; , &quot;).filter();PF(&quot; , &quot;'&quot; , &quot;NewInvoiceWid&quot; , &quot;'&quot; , &quot;).hide();PF(&quot; , &quot;'&quot; , &quot;confirmationdlg&quot; , &quot;'&quot; , &quot;).hide();}else{PF(&quot; , &quot;'&quot; , &quot;confirmationdlg&quot; , &quot;'&quot; , &quot;).show();} handleComplete(xhr,status,args);;},pa:arguments[0]});}Update Visitui-buttonHide 


LOINCTextFileFDIFinancialGroupingResultExcludeFromDRGERXRisk of MortalityProgramACHI_MAPPINGLOINCObservations



                        
                            


Observation Type
Observation Code
ObservationValue
Value Type




LOINCTextFileFDIFinancialGroupingResultExcludeFromDRGERXRisk of MortalityProgramACHI_MAPPINGLOINC
LOINCLOINC





   
                            
                            Add Observation
                            
                        file...Observation TypeObservation CodeObservationValueValue TypeActionsNo record found.CloseSave
Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesNotes








Note Type


Missing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesMissing Authorisation







Note Title










Add Note













Note Description








Note TypeNote TitleNote DateNote DescriptionActionsNo record found.CloseSave
id(&quot;InsuranceDetailsForm&quot;)/div[1]Export as ExcelDay Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital.Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular SurgeryAllInsuranceCashAllNo servicesBilledPartially BilledRefundedNot Billed... SAAUDI(The Mediterranean &amp; Gulf Cooperative Insurance &amp; Reinsurance Company (MedGulf???????????Accumed Comany123132020-NACIG - TCSACIG- MEDNETAL ETIHADAl Sagr Cooperative  Insurance CompanyALBAHAALFA131ALJazira TakafulALJUMAIH COallamAllianz Saudi Fransi Insurance CompanyAlrajhi Takaful for Cooperative InsuranceAMANA INSURANCEARABIA InsuranceARABIAN SHEILDARAMCOAXA Cooperative Insurance CompanybandrbupaBupaBUPA Arabia for Cooperative InsuranceBURUJDate SeptEnvironmentETIHADfgfdgfvdgcfhgnvbGENERAL 2021GENERAL-2020-JgggGLOBEMEDGLOBEMED SAUDI ( GMS )GLOBEMED SAUDI ( GMS )GM-AMINAHGOSIGULF GENERAL - MEDNETGULF GENERAL - TCSGulf general Insurance CompanyGulf unionGulf Union Cooperative Insurance CompanytesthghgfdfdHKK Coinsurance company nameinsuranceTestJPJubil InsKHAMISKSAKSAKSA001Malath Cooperative Insurance &amp; Reinsurance CompanymaxmazMEDNETMOUNIRMS-NIZIHAmyNextcareRTRSAGR - MEDNETSALAMA INSURANCEsaudiSaudi Arabian cooperative Insurance Company (Saico)Saudi Enaya Cooperative InsuranceSaudi United  Cooperative Insurance  (WALA&quot; , &quot;'&quot; , &quot;A)sdfsSelf Pay - CashsolidaritysosoSPSRSYTawuniya cooperative  Insurance CompanyTawuniya cooperative  Insurance CompanyffgTCStest2020testAL RAJHItestAL SAGR - NEXTCAREtestMalathtestMalath CooperativetestMedGulftestRAJHIAAtestSAICOtestSAUDI ENAYAtestSGH STAFF - TAWUNIYAtestTAWUNIYAtestTawuniya cooperative Insurance CompanytestWALAAtestWALAA - NEXTCAREToday-27-07TOKIO MARINETOTAL CARE TOTAL CARE SAUDI (TCS)TRADE UNIONUAEUCAUNSPECIFIEDyamamahytPayer Patient Details


StatusPatientClassPolicy Holder NameDenial CodeReference NumberStatus DescriptionAdditional InfoCoveragesUpdate PatientHide 
...FemaleMaleAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWESelect One...DivorcedMarriedSeparatedSingleUnknownWidowedCreate Patient





MRN*National ID







Name*Middlename*Surname*Arabic First NameArabic Middle NameArabic Sure NameGender*...FemaleMale...Passport IDDOB / Age :*NationalityAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWEAFGHANISTANMarital StatusSelect One...DivorcedMarriedSeparatedSingleUnknownWidowedSelect One...HomeOfficeFaxMobilePersonEmailAdd Insurance Card



Member IDPayer IDPolicy/ClassStart DateEnd DateEditDeleteNo record found.


insurance  inquiry  request
Insert Patient
Cancel







cchi eligibility RequestPayers info


#InsurersRetry AllMessageCard NumberEffective DateExpiry DatePayerPlanNameMessageNo records found.



ClosePrimeFaces.cw(&quot;Poll&quot;,&quot;checkPayersThreads&quot;,{id:&quot;dhaInfoPanelForm:checkPayersThreads&quot;,frequency:5,autoStart:false,fn:function(){PrimeFaces.ab({s:&quot;dhaInfoPanelForm:checkPayersThreads&quot;,f:&quot;dhaInfoPanelForm&quot;,u:&quot;dhaInfoPanelForm:datalistAvailableEligibility&quot;,g:false,d:500});}});
...FemaleMaleAFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWESelect One...DivorcedMarriedSeparatedSingleUnknownWidowedEdit Patient


MRN*National ID







Name*






Middlename*Surname*






Arabic First NameArabic Middle NameArabic Sure NameGender*


...FemaleMale...



Passport IDDOB / Age :*






Nationality


AFGHANISTANALAND ISLANDSALBANIAALGERIAAMERICAN SAMOAANDORRAANGOLAANGUILLAANTARCTICAANTIGUA AND BARBUDAARGENTINAARMENIAARUBAAUSTRALIAAUSTRIAAZERBAIJANBAHAMASBAHRAINBANGLADESHBARBADOSBELARUSBELGIUMBELIZEBENINBERMUDABHUTANBOLIVIA, PLURINATIONAL STATE OFBONAIRE, SINT EUSTATIUS AND SABABOSNIA AND HERZEGOVINABOTSWANABOUVET ISLANDBRAZILBRITISH INDIAN OCEAN TERRITORYBRUNEI DARUSSALAMBULGARIABURKINA FASOBURUNDICAMBODIACAMEROONCANADACAPE VERDECAYMAN ISLANDSCENTRAL AFRICAN REPUBLICCHADCHILECHINACHRISTMAS ISLANDCOCOS (KEELING) ISLANDSCOLOMBIACOMOROSCONGOCONGO, THE DEMOCRATIC REPUBLIC OF THECOOK ISLANDSCOSTA RICACOTE D&quot; , &quot;'&quot; , &quot;IVOIRECROATIACUBACURACAOCYPRUSCZECH REPUBLICDENMARKDJIBOUTIDOMINICADOMINICAN REPUBLICECUADOREGYPTEL SALVADOREQUATORIAL GUINEAERITREAESTONIAETHIOPIAFALKLAND ISLANDS (MALVINAS)FAROE ISLANDSFIJIFINLANDFRANCEFRENCH GUIANAFRENCH POLYNESIAFRENCH SOUTHERN TERRITORIESGABONGAMBIAGEORGIAGERMANYGHANAGIBRALTARGREECEGREENLANDGRENADAGUADELOUPEGUAMGUATEMALAGUERNSEYGUINEAGUINEA-BISSAUGUYANAHAITIHEARD ISLAND AND MCDONALD ISLANDSHOLY SEE (VATICAN CITY STATE)HONDURASHONG KONGHUNGARYICELANDINDIAINDONESIAIRAN (ISLAMIC REPUBLIC OF)IRAQIRELANDISLE OF MANISRAELITALYJAMAICAJAPANJERSEYJORDANKAZAKHSTANKENYAKIRIBATIKOREA, DEMOCRATIC PEOPLE&quot; , &quot;'&quot; , &quot;S REPUBLIC OFKOREA, REPUBLIC OFKUWAITKYRGYZSTANLAO PEOPLE&quot; , &quot;'&quot; , &quot;S DEMOCRATIC REPUBLICLATVIALEBANONLESOTHOLIBERIALIBYALIECHTENSTEINLITHUANIALUXEMBOURGMACAOMACEDONIA, THE FORMER YUGOSLAV REPUBLIC OFMADAGASCARMALAWIMALAYSIAMALDIVESMALIMALTAMARSHALL ISLANDSMARTINIQUEMAURITANIAMAURITIUSMAYOTTEMEXICOMICRONESIA, FEDERATED STATES OFMOLDOVA, REPUBLIC OFMONACOMONGOLIAMONTENEGROMONTSERRATMOROCCOMOZAMBIQUEMYANMARNAMIBIANAURUNEPALNETHERLANDSNEW CALEDONIANEW ZEALANDNICARAGUANIGERNIGERIANIUENORFOLK ISLANDNORTHERN MARIANA ISLANDSNORWAYNOT APPLICABLEOMANPAKISTANPALAUPALESTINIAN TERRITORY, OCCUPIEDPANAMAPAPUA NEW GUINEAPARAGUAYPERUPHILIPPINESPITCAIRNPOLANDPORTUGALPUERTO RICOQATARREUNIONROMANIARUSSIAN FEDERATIONRWANDASAINT BARTHELEMYSAINT HELENA, ASCENSION AND TRISTAN DA CUNHASSAINT KITTS AND NEVISSAINT LUCIASAINT MARTIN (FRENCH PART)SAINT PIERRE AND MIQUELONSAINT VINCENT AND THE GRENADINESSAMOASAN MARINOSAO TOME AND PRINCIPESAUDI ARABIASENEGALSERBIASEYCHELLESSIERRA LEONESINGAPORESINT MAARTEN (DUTCH PART)SLOVAKIASLOVENIASOLOMON ISLANDSSOMALIASOUTH AFRICASOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDSSOUTH SUDANSPAINSRI LANKASUDANSURINAMESVALBARD AND JAN MAYENSWAZILANDSWEDENSWITZERLANDSYRIAN ARAB REPUBLICTAIWAN, PROVINCE OF CHINATAJIKISTANTANZANIA, UNITED REPUBLIC OFTHAILANDTIMOR-LESTETOGOTOKELAUTONGATRINIDAD AND TOBAGOTUNISIATURKEYTURKMENISTANTURKS AND CAICOS ISLANDSTUVALUUGANDAUKRAINEUNITED ARAB EMIRATESUNITED KINGDOMUNITED STATESUNITED STATES MINOR OUTLYING ISLANDSURUGUAYUZBEKISTANVANUATUVATICAN CITY STATE (HOLY SEE)VENEZUELA, BOLIVARIAN REPUBLIC OFVIET NAMVIRGIN ISLANDS (BRITISH)VIRGIN ISLANDS (U.S.)WALLIS AND FUTUNAWESTERN SAHARAYEMENYUGOSLAVIAZAMBIAZIMBABWEAFGHANISTAN



Marital StatusSelect One...DivorcedMarriedSeparatedSingleUnknownWidowedSelect One...Home






OfficeFaxMobile






PersonEmail






Add Insurance CardMember IDPayer IDPolicy/ClassStart DateEnd DateEditDeleteNo record found.


Save
Cancel




Create Patient Card


GeneralInformationCopayment And DeductibleImages And other Information 


OP Copayment:

OP MaxPatientShare:



IP Copayment:

IP MaxPatientShare:




CopaymentCopayment CategoryCoPaymentAmountCeilingIP/OPEditDeleteNo record found.CreateDeductibleCopayment CategoryDeductableCeilingIP/OPEditDeleteNo record found.CreateCard Coverages*Covered ServicesNone Covered


DentalMaternityOptical












Foreground





















Background 


















Card Notes


Save
Cancel




ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate CoPayment


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitCoPayment Value*



%



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate Deductible


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitDeductible Value*



SAR



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate CoPayment


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitCoPayment Value*



%



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomIP/OPIPOPCreate Deductible


The Name field is required.ART Care UnitAccomdation ChargesCARDIC SYRGERYCHEMOTHERAPYCardiac SciencesDentalDermatological and plastic proceduresDermatologyDietary DepartmentEEGENTEmbalmingEndocrinologt  D.MEndoscopyGeneral SurgeryGynaecological proceduresHome VisitICU &amp; NICUImaging servicesInternal MedicineJob ServicesKSA Service CodesLaboratoryLaboratory and Pathology ServicesMedical RehabilitaionNephrology / Renal DialysisNeuro SurgeryNeurophysiologyNon-invasive, cognitive and other interventions, not elsewhere classified**   ORTHOPEDIC SURGERYObsterics / GynacologyObstetric proceduresOphthalomologyOutpatient &amp; Emergency ServicesP.H.C.C ConsPSYCHIATRICPackage DealPharmacyPhysical TherapyPlastic SurgeryProcedures on blood and blood-forming organsProcedures on breastProcedures on cardiovascular systemProcedures on digestive systemProcedures on ear and mastoid processProcedures on endocrine systemProcedures on eye and adnexaProcedures on male genital organsProcedures on musculoskeletal systemProcedures on nervous systemProcedures on nose, mouth and pharynxProcedures on respiratory systemProcedures on urinary systemPulmonary / RespiratoryRadiation oncology proceduresRadiologyRoom &amp; BoardSupport ServicesToxicology and Forensic ChemistryTransportationVACCINATIONVascluer Surgerypediatric surgeryplaster roomART Care UnitDeductible Value*



SAR



CeilingIP/OP:IP/OPIPOPIP/OPSaveCancel
Edit Patient Card


GeneralInformationCopayment And Deductible Images And other Information


OP Copayment:

OP MaxPatientShare:



IP Copayment:

IP MaxPatientShare:



Prior Approval Limit:




CopaymentCopayment CategoryCoPaymentAmountCeilingApproval LimitIP/OPEditDeleteNo record found.CreateDeductibleCopayment CategoryDeductableCeilingApproval LimitIP/OPEditDeleteNo record found.CreateCard Coverages*Covered ServicesNone Covered


DentalMaternityOptical












Foreground





















Background


















Card Notes


Save
Cancel




Existing Patients


Similar patient records found based on values entered, to proceed please choose one of the following:Select from existing patient recordCreate new patient record


National IDNameMobileDOBMember IDStart DateExpiry DateNo records found.



ConfirmCancel
Insurance Details



Valid InsurancesExpired InsurancesNot Verified InsurancesCard NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.Patient Insurance NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.Patient Insurance NumberInsuranceLisenceInsurance NamePolicyNumber/ClassStart Date &amp; TimeEnd Date &amp; TimeCard NotesEditNo record found.SaveAdd Insuranceinsurance  inquiry  requestCancel
CCHI Eligiblity Information


#PatientGenderNational IDpolicyNumberClassEffective DateExpiry DateCopayment And DeductibleIP MaxPatientShare:Payer NameCCHI infoNaphis InfoNo records found.HideUpdate Patient Alert






Patient exists in our system is differnet than one retreived from payer !





Existing Patient :
FGMNDCFHS EDGFRTHJU


Payer Patient: 










Update Patient
Cancel 












Payer Info            time out






Request to payer time out




                        


OK




                        
Insurance Co not found






Payer is not exist in data base please add it




                        
                           


OK




                           
detailsMissing AuthorisationNursing QueriesIP PreauthOP PreauthDr. NotesCSR/Card RelatedRadiologyMissing Lab ReportsOT NotesCath Lab/Procedure ReportsPharmacyPhysiotherapyPre-Auth RadiologyConsumablesDischarge reportGeneralLaboratoryOthersPrescriptionRadiologySurgeryPayer Patient Details




Payer Patient Details



Please check member eligibility as per the date of serviceNotes...Allergy/ImunologyArtificial LimbsBusiness Trip Check upCardiologyCardiothoracic SurgeryDentalDermaVenDialysisENTEmergencyEndocrinologyFamily PracticeGastroenterologyGeneral Medical Check upGeneral PracticeGeneral SurgeryGerontologyGynecologyHearing AidsHematologyHepatologyHome Health CareIndustrial Check upInfectious DiseasesInpatientInternal MedicineIqama Issuance and  Renewal Check upMaternityNeonatal-Perinatal MedicineNephrologyNeuro-SurgeryNeurologyNon electronic wheel chairNuclear MedicineObstetricsOncologyOpthalmicOpticalOral/Maxillofacial SurgeryOrthopedicOutpatientPaediatricsPediatric SurgeryPharmacyPhysiotherapyPlastic SurgeryPre-Employment Check upPre-Marital Check upPre-School Check upPreventive CarePsychiatryPulmonologyRheumatologySteam DeviceSugar Measuring DeviceUrologyVascular Surgery...Day Case - patient occupies bed and is discharged on the same dayEmergency - patient is seen in Emergency Department and is not admitted to Inpatient BedHome Health Care – patient is seen in the homeHospital Inpatient - patient occupies inpatient bed and stays overnightOutpatient - patient not admitted and not seen in Emergency
DepartmentPre-AdmissionPrimary Health Care – patient is seen in a facility/centre outside of the hospital....Elective waiting list admission government free SchemeElective waiting list admission insurance coverage SchemeElective waiting list admission self-payment SchemeEmergency Admission by insurance companyEmergency Admission by referral from general government hospitalEmergency Admission by referral from government primary healthcare centerEmergency Admission by referral from private hospitalEmergency Admission by referral from private primary healthcare centerEmergency Admission from hospital EREmergency Admission from hospital outpatientEmergency Maternity Birth AdmissionImmediate AdmissionOthersPlanned Maternity Birth AdmissionUnknown...Discharge /Transfer to Other Health Care AccommodationDischarge /Transfer to a Residential Ageing ServiceDischarge/Transfer to a Psychiatric HospitalDischarge/transfer to an Acute HospitalHome/OtherStatistical Discharge-Type ChangeInitial client service eventSubsequent client service eventAcute CareBoarderComplex MaintenanceGeneral MaintenancePalliativeRehabilitation...Follow upNewReferralRefillWalk InAdmitted to this hospitalDead on arrivalDid not wait to be attended by a health care professionalDied in EDLeft at own risk after being attended by a health care professional, but before the non-admitted patient ED service episode was completedNon-admitted patient ED episode completed -referred to another hospital for admission;Non-admitted patient ED episode completed-departed without being admitted or referred to another hospitalRegistered, advised of another health care service, and left the EDwithout being attended by a health care professionalImmediate resuscitationNon-UrgentStandard ERUrgentVery UrgentadmittingclinicaldifferentialdischargelaboratorynursingprenatalprincipalradiologyremoteretrospectivesecondaryselfCondition not noted as arising during the episode of admitted patient careCondition with onset during the episode of admitted patient careNot reportedPackageTabletOther ServicePrincipalSecondaryCheck-upChronicCongenitalDental CleaningInfertilityMaternity ComplicationsOthersPregnancyPsychiatricRTATreatmentVaccinationWork RelatedDayMonthWeekYearwarning






Validation error, Continue!




                        
                         
                        
                        ConfirmedCancel
&quot;))]</value>
      <webElementGuid>74bbab0b-4d33-464f-a997-63c275444871</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
