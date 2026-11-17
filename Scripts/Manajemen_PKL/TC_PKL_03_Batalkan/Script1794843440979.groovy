import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
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

WebUI.openBrowser(null)

WebUI.navigateToUrl('http://localhost/e-pkl/e-pkl/admin-web/public/?page=manajemen-pkl')

WebUI.setText(findTestObject('Manajemen_PKL_03/Page_Login Admin  E-PKL/input_Masukkan username atau email'), 'admin')

WebUI.setEncryptedText(findTestObject('Manajemen_PKL_03/Page_Login Admin  E-PKL/input_Masukkan password'), '8SQVv/p9jVQa8N8i3eDN3paNTjSkKFPY')

WebUI.click(findTestObject('Manajemen_PKL_03/Page_Login Admin  E-PKL/button_Masuk'))

WebUI.click(findTestObject('Manajemen_PKL_03/Page_Dashboard Monitoring PKL  E-PKL/span_Manajemen PKL'))

WebUI.click(findTestObject('Manajemen_PKL_03/Page_Manajemen PKL  E-PKL/a_Detail'))

WebUI.selectOptionByValue(findTestObject('Manajemen_PKL_03/Page_Detail Penempatan PKL  E-PKL/select_Alasan pembatalan'), 
    'instansi_tidak_dapat_menerima_siswa', false)

WebUI.click(findTestObject('Manajemen_PKL_03/Page_Detail Penempatan PKL  E-PKL/button_Batalkan'))

WebUI.closeBrowser()

