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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://inixindojogja.com/api/')

WebUI.setText(findTestObject('Object Repository/TC admin delete user/Page_DummyLog/input_INIX_username'), 'admin@oti.com')

WebUI.setEncryptedText(findTestObject('Object Repository/TC admin delete user/Page_DummyLog/input_INIX_password'), '8ylGA/40/qI=')

WebUI.click(findTestObject('Object Repository/TC admin delete user/Page_DummyLog/button_Sign In'))

WebUI.click(findTestObject('Object Repository/TC admin delete user/Page_AdminLTE 2  Blank Page/span_Dashboard'))

WebUI.click(findTestObject('Object Repository/TC admin delete user/Page_AdminLTE 2  Blank Page/a_Data user'))

WebUI.click(findTestObject('Object Repository/TC admin delete user/Page_AdminLTE 2  Blank Page/div_Responsive Hover Table                 _41d0a1'))

WebUI.verifyElementPresent(findTestObject('TC admin delete user/Page_AdminLTE 2  Blank Page/td_muhammadyasinoti.com'), 0)

WebUI.click(findTestObject('TC admin delete user/Page_AdminLTE 2  Blank Page/tong sampah yasin'))

WebUI.acceptAlert(FailureHandling.STOP_ON_FAILURE)

WebUI.verifyTextPresent('Data Berhasil Di Hapus', true)

WebUI.closeBrowser()

